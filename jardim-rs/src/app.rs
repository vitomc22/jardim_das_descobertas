//! Interface (egui, modo imediato): só redesenha quando há toque/animação.
//! Lógica de navegação equivalente a `app.js`.

use crate::art;
use crate::curriculum::{self as cur, Question, Scene, Visual, CATEGORIES, GROUPS};
use crate::raster::Raster;
use crate::rng;
use crate::store::{Store, GOAL};
use crate::tts::Tts;
use egui::{
    pos2, vec2, Align, Align2, Color32, CornerRadius, FontFamily, FontId, Layout, Pos2, Rect, Response, Sense, Stroke,
    StrokeKind, Ui, Vec2,
};
use std::collections::HashSet;

// ------------------------------------------------------------- paleta (style.css)
const INK: Color32 = Color32::from_rgb(0x29, 0x4b, 0x43);
const MUTED: Color32 = Color32::from_rgb(0x6d, 0x7d, 0x70);
const PAPER: Color32 = Color32::from_rgb(0xf8, 0xf6, 0xee);
const GREEN: Color32 = Color32::from_rgb(0x34, 0x7b, 0x65);
const GREEN_SHADOW: Color32 = Color32::from_rgb(0x26, 0x61, 0x4f);
const LINE: Color32 = Color32::from_rgb(0xe2, 0xe6, 0xd9);
const QUIET: Color32 = Color32::from_rgb(0xee, 0xee, 0xe4);
const QUIET_BORDER: Color32 = Color32::from_rgb(0xde, 0xdf, 0xd2);
const EYEBROW: Color32 = Color32::from_rgb(0x7f, 0x8c, 0x73);
const EM: Color32 = Color32::from_rgb(0x56, 0x85, 0x6b);
const SUCCESS: Color32 = Color32::from_rgb(0x38, 0x76, 0x57);

pub fn hex(s: &str) -> Color32 {
    let s = s.trim_start_matches('#');
    let p = |i: usize| u8::from_str_radix(s.get(i..i + 2).unwrap_or("00"), 16).unwrap_or(0);
    Color32::from_rgb(p(0), p(2), p(4))
}

fn card_bg(color: &str) -> Color32 {
    hex(match color {
        "blue" => "#deedf2",
        "yellow" => "#faedc8",
        "green" => "#e0eddc",
        "purple" => "#e8e2f1",
        "peach" => "#f6e7d5",
        _ => "#f7e1db",
    })
}

fn serif(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name("serif".into()))
}
fn sans(size: f32) -> FontId {
    FontId::new(size, FontFamily::Proportional)
}

// ------------------------------------------------------------- estado de tela

#[derive(PartialEq, Clone, Copy)]
enum Screen {
    Home,
    Game,
    Finish,
}

enum Modal {
    None,
    Gate { a: u32, b: u32, input: String, error: bool },
    Parents,
    ResetConfirm,
}

enum Feedback {
    Idle,
    Hint,
    Retry,
    Success,
}

struct Session {
    game: &'static str,
    round: u32,
    length: u32,
    q: Question,
    correct: bool,
    revealed: Vec<usize>,
    matched: Vec<usize>,
    flip_back_at: Option<f64>,
    feedback: Feedback,
    wiggle: Option<(usize, f64)>,
    counted: HashSet<usize>,
}

#[derive(Clone)]
enum Act {
    Home,
    Explore,
    Category(&'static str),
    Age(&'static str),
    Start(&'static str),
    Answer(usize),
    Flip(usize),
    Listen,
    Hint,
    Next,
    CountItem(usize),
    Voice,
    Gate,
    Close,
    ParentAge(&'static str),
    ParentVoice,
    ResetConfirm,
    Parents,
    Reset,
    GateKey(char),
    GateSubmit,
}

pub struct JardimApp {
    store: Store,
    raster: Raster,
    tts: Option<Tts>,
    screen: Screen,
    modal: Modal,
    category: &'static str,
    entry_seed: u32,
    session: Option<Session>,
    acts: Vec<Act>,
    hidden: bool,
    toast: Option<(String, f64)>,
    scroll_top: bool,
    scroll_activities: bool,
    warmed_for: Option<(String, u32)>,
    android: winit::platform::android::activity::AndroidApp,
}

impl JardimApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        store: Store,
        tts: Option<Tts>,
        android: winit::platform::android::activity::AndroidApp,
    ) -> Self {
        install_fonts(&cc.egui_ctx);
        let mut style = (*cc.egui_ctx.style()).clone();
        style.visuals = egui::Visuals::light();
        style.visuals.panel_fill = PAPER;
        style.visuals.override_text_color = Some(INK);
        style.animation_time = 0.08;
        style.spacing.scroll = egui::style::ScrollStyle::floating();
        cc.egui_ctx.set_style(style);
        let mut app = JardimApp {
            store,
            raster: Raster::new(),
            tts,
            screen: Screen::Home,
            modal: Modal::None,
            category: "all",
            entry_seed: rng::entry_seed(),
            session: None,
            acts: Vec::new(),
            hidden: false,
            toast: None,
            scroll_top: true,
            scroll_activities: false,
            warmed_for: None,
            android,
        };
        app.store.state.ensure_day();
        app
    }

    fn speak(&mut self, text: &str) {
        if !self.store.state.voice {
            return;
        }
        if let Some(t) = &mut self.tts {
            t.speak(text);
        }
    }
    fn stop_voice(&mut self) {
        if let Some(t) = &mut self.tts {
            t.stop();
        }
    }

    fn level(&self) -> usize {
        cur::group_index(&self.store.state.group)
    }

    fn catalog(&self) -> Vec<&'static cur::Game> {
        let s = &self.store.state;
        cur::catalog(self.level(), rng::seed_for(&s.daily.date, &s.group, "catalog", 0, self.entry_seed))
    }

    // --------------------------------------------------------- ações (app.js)

    fn go_home(&mut self) {
        self.store.state.ensure_day();
        self.stop_voice();
        self.session = None;
        self.screen = Screen::Home;
        self.modal = Modal::None;
        self.scroll_top = true;
        self.raster.trim(400);
    }

    fn start(&mut self, game: &'static str) {
        self.store.state.ensure_day();
        self.screen = Screen::Game;
        self.modal = Modal::None;
        let d = self.store.state.daily(game);
        let length = if d.completed { GOAL } else { GOAL - d.solved };
        self.session = Some(Session {
            game,
            round: 0,
            length,
            q: cur::generate(game, "little", 0),
            correct: false,
            revealed: vec![],
            matched: vec![],
            flip_back_at: None,
            feedback: Feedback::Idle,
            wiggle: None,
            counted: HashSet::new(),
        });
        self.next_question();
    }

    fn next_question(&mut self) {
        self.store.state.ensure_day();
        let Some(game) = self.session.as_ref().map(|s| s.game) else { return };
        let st = self.store.state.stat_mut(game);
        st.rounds += 1;
        let rounds = st.rounds;
        self.store.save();
        let s = &self.store.state;
        let seed = rng::seed_for(&s.daily.date, &s.group, game, rounds, self.entry_seed);
        let q = cur::generate(game, &s.group.clone(), seed);
        let say = format!("{}{}", q.story.map(|t| format!("{t} ")).unwrap_or_default(), q.prompt);
        if let Some(sess) = &mut self.session {
            sess.q = q;
            sess.correct = false;
            sess.revealed.clear();
            sess.matched.clear();
            sess.flip_back_at = None;
            sess.feedback = Feedback::Idle;
            sess.wiggle = None;
            sess.counted.clear();
        }
        self.scroll_top = true;
        self.speak(&say);
    }

    fn record_try(&mut self) {
        if let Some(g) = self.session.as_ref().map(|s| s.game) {
            self.store.state.stat_mut(g).tries += 1;
            self.store.save();
        }
    }

    fn correct(&mut self) {
        let Some(s) = &mut self.session else { return };
        if s.correct {
            return;
        }
        s.correct = true;
        s.feedback = Feedback::Success;
        let g = s.game;
        self.store.state.ensure_day();
        self.store.state.record_daily(g);
        self.store.state.stat_mut(g).solved += 1;
        self.store.save();
        self.speak("Você descobriu! Muito bem.");
    }

    fn answer(&mut self, i: usize, now: f64) {
        let Some(s) = &self.session else { return };
        if s.correct {
            return;
        }
        self.record_try();
        let s = self.session.as_mut().unwrap();
        if i == s.q.answer {
            self.correct();
        } else {
            s.wiggle = Some((i, now + 0.4));
            s.feedback = Feedback::Retry;
            let h = format!("Vamos tentar de novo? {}", s.q.hint);
            self.speak(&h);
        }
    }

    fn flip(&mut self, i: usize, now: f64) {
        let Some(s) = &mut self.session else { return };
        if s.flip_back_at.is_some() || s.correct || s.revealed.contains(&i) || s.matched.contains(&i) {
            return;
        }
        s.revealed.push(i);
        if s.revealed.len() < 2 {
            return;
        }
        self.record_try();
        let s = self.session.as_mut().unwrap();
        let (a, d) = (s.revealed[0], s.revealed[1]);
        if s.q.cards[a].pair == s.q.cards[d].pair {
            s.matched.extend([a, d]);
            s.revealed.clear();
            if s.matched.len() == s.q.cards.len() {
                self.correct();
            } else {
                self.speak("Um par!");
            }
        } else {
            s.flip_back_at = Some(now + 1.2);
        }
    }

    fn finish(&mut self) {
        self.store.state.ensure_day();
        self.stop_voice();
        let Some(s) = &self.session else { return };
        let g = s.game;
        let len = s.length;
        self.store.state.stat_mut(g).sets += 1;
        self.store.save();
        self.screen = Screen::Finish;
        self.scroll_top = true;
        let headline = if len == 1 { "Uma nova descoberta!".to_string() } else { format!("{len} novas descobertas!") };
        self.speak(&format!("{headline} Que tal fazer uma pausa?"));
    }

    fn back(&mut self, ctx: &egui::Context) {
        if !matches!(self.modal, Modal::None) {
            self.modal = Modal::None;
        } else if self.screen != Screen::Home {
            self.go_home();
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    fn apply(&mut self, act: Act, now: f64) {
        self.store.state.ensure_day();
        match act {
            Act::Home => self.go_home(),
            Act::Explore => {
                self.scroll_activities = true;
                self.speak("Escolha uma atividade para começar.");
            }
            Act::Category(c) => {
                self.category = c;
                self.scroll_activities = true;
            }
            Act::Age(g) => {
                self.category = "all";
                self.store.state.group = g.into();
                self.store.save();
                self.scroll_activities = true;
            }
            Act::Start(g) => self.start(g),
            Act::Answer(i) => self.answer(i, now),
            Act::Flip(i) => self.flip(i, now),
            Act::Listen => {
                if let Some(s) = &self.session {
                    let t = format!("{}{}", s.q.story.map(|t| format!("{t} ")).unwrap_or_default(), s.q.prompt);
                    self.speak(&t);
                }
            }
            Act::Hint => {
                if let Some(s) = &mut self.session {
                    if !s.correct {
                        s.feedback = Feedback::Hint;
                    }
                    let t = format!("{}{}", s.q.story.map(|t| format!("{t} ")).unwrap_or_default(), s.q.hint);
                    self.speak(&t);
                }
            }
            Act::Next => {
                if let Some(s) = &mut self.session {
                    if s.correct {
                        s.round += 1;
                        if s.round == s.length {
                            self.finish();
                        } else {
                            self.next_question();
                        }
                    }
                }
            }
            Act::CountItem(i) => {
                if let Some(s) = &mut self.session {
                    if !s.counted.remove(&i) {
                        s.counted.insert(i);
                    }
                }
            }
            Act::Voice | Act::ParentVoice => {
                self.store.state.voice = !self.store.state.voice;
                self.store.save();
                if !self.store.state.voice {
                    self.stop_voice();
                } else if let (Act::Voice, Some(s)) = (&act, &self.session) {
                    let p = s.q.prompt.clone();
                    self.speak(&p);
                }
            }
            Act::Gate => {
                self.stop_voice();
                let r = rng::entry_seed();
                self.modal = Modal::Gate { a: 6 + r % 4, b: 6 + (r / 4) % 4, input: String::new(), error: false };
            }
            Act::GateKey(c) => {
                if let Modal::Gate { input, error, .. } = &mut self.modal {
                    *error = false;
                    if c == '<' {
                        input.pop();
                    } else if input.len() < 3 {
                        input.push(c);
                    }
                }
            }
            Act::GateSubmit => {
                if let Modal::Gate { a, b, input, error } = &mut self.modal {
                    if input.parse::<u32>().ok() == Some(*a * *b) {
                        self.modal = Modal::Parents;
                    } else {
                        *error = true;
                    }
                }
            }
            Act::Close => {
                self.modal = Modal::None;
            }
            Act::ParentAge(g) => {
                self.store.state.group = g.into();
                self.category = "all";
                self.store.save();
            }
            Act::ResetConfirm => self.modal = Modal::ResetConfirm,
            Act::Parents => self.modal = Modal::Parents,
            Act::Reset => {
                self.store.state.stats.clear();
                self.store.state.daily.activities.clear();
                self.store.save();
                self.modal = Modal::Parents;
            }
        }
    }

    // --------------------------------------------------------- desenho

    /// Desenha um SVG (rasterizado/cacheado) dentro de `rect`.
    fn svg(&mut self, ui: &Ui, svg: &str, rect: Rect) {
        let ppp = ui.ctx().pixels_per_point();
        let tex = self.raster.get(ui.ctx(), svg, rect.width() * ppp, rect.height() * ppp);
        ui.painter().image(tex.id(), rect, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
    }

    fn svg_rotated(&mut self, ui: &Ui, svg: &str, rect: Rect, deg: f32) {
        let ppp = ui.ctx().pixels_per_point();
        let tex = self.raster.get(ui.ctx(), svg, rect.width() * ppp, rect.height() * ppp);
        let img = egui::Image::new(egui::load::SizedTexture::new(tex.id(), rect.size()))
            .rotate(deg.to_radians(), vec2(0.5, 0.5));
        img.paint_at(ui, rect);
    }

    /// Parte de grade de rasterização paralela: todos os SVGs que a tela vai usar.
    fn prewarm_home(&mut self, ctx: &egui::Context, card_w: f32) {
        let key = (self.store.state.group.clone(), (card_w * 10.0) as u32);
        if self.warmed_for.as_ref() == Some(&key) {
            return;
        }
        let ppp = ctx.pixels_per_point();
        let mut jobs: Vec<(String, f32, f32)> = vec![(art::garden(), 460.0 * ppp, 265.0 * ppp), (art::icon("leaf", None), 34.0 * ppp, 34.0 * ppp)];
        for g in cur::GAMES.iter() {
            let (w, h) = tile_size(g.icon);
            jobs.push((art::tile(g.icon), w * ppp, h * ppp));
        }
        self.raster.prewarm(ctx, &jobs);
        self.warmed_for = Some(key);
    }

    fn prewarm_question(&mut self, ctx: &egui::Context) {
        let Some(s) = &self.session else { return };
        let ppp = ctx.pixels_per_point();
        let c = CHOICE_ART * ppp;
        let mut jobs: Vec<(String, f32, f32)> = s.q.options.iter().filter_map(|o| visual_svg(&o.visual)).map(|svg| (svg, c, c)).collect();
        for card in &s.q.cards {
            jobs.push((art::icon(card.kind, None), c, c));
        }
        self.raster.prewarm(ctx, &jobs);
    }

    fn quiet_button(&mut self, ui: &mut Ui, text: &str, act: Act) -> Response {
        let font = sans(15.0);
        let galley = ui.painter().layout_no_wrap(text.to_owned(), font, INK);
        let size = vec2(galley.size().x + 36.0, 46.0);
        let (rect, resp) = ui.allocate_exact_size(size, Sense::click());
        let p = ui.painter();
        let fill = if resp.is_pointer_button_down_on() { LINE } else { QUIET };
        p.rect_filled(rect, CornerRadius::same(23), fill);
        p.rect_stroke(rect, CornerRadius::same(23), Stroke::new(1.0, QUIET_BORDER), StrokeKind::Inside);
        p.galley(rect.center() - galley.size() / 2.0, galley, INK);
        if resp.clicked() {
            self.acts.push(act);
        }
        resp
    }

    fn primary_button(&mut self, ui: &mut Ui, text: &str, act: Act) -> Response {
        let galley = ui.painter().layout_no_wrap(text.to_owned(), serif(18.0), Color32::WHITE);
        let size = vec2(galley.size().x + 48.0, 56.0);
        let (rect, resp) = ui.allocate_exact_size(size, Sense::click());
        let down = resp.is_pointer_button_down_on();
        let body = if down { rect.translate(vec2(0.0, 3.0)) } else { rect };
        let p = ui.painter();
        p.rect_filled(rect.translate(vec2(0.0, 4.0)), CornerRadius::same(16), GREEN_SHADOW);
        p.rect_filled(body, CornerRadius::same(16), GREEN);
        p.galley(body.center() - galley.size() / 2.0, galley, Color32::WHITE);
        if resp.clicked() {
            self.acts.push(act);
        }
        resp
    }

    fn pill(&mut self, ui: &mut Ui, text: &str, selected: bool, act: Act) {
        let galley = ui.painter().layout_no_wrap(text.to_owned(), sans(15.0), if selected { Color32::WHITE } else { MUTED });
        let (rect, resp) = ui.allocate_exact_size(vec2(galley.size().x + 30.0, 44.0), Sense::click());
        if selected {
            ui.painter().rect_filled(rect, CornerRadius::same(22), GREEN);
        }
        ui.painter().galley(rect.center() - galley.size() / 2.0, galley, Color32::WHITE);
        if resp.clicked() {
            self.acts.push(act);
        }
    }

    fn tab(&mut self, ui: &mut Ui, text: &str, active: bool, act: Act) {
        let color = if active { hex("#346950") } else { MUTED };
        let galley = ui.painter().layout_no_wrap(text.to_owned(), sans(15.0), color);
        let (rect, resp) = ui.allocate_exact_size(vec2(galley.size().x + 30.0, 42.0), Sense::click());
        let p = ui.painter();
        if active {
            p.rect_filled(rect, CornerRadius::same(21), hex("#e0eada"));
            p.rect_stroke(rect, CornerRadius::same(21), Stroke::new(1.0, hex("#9dba99")), StrokeKind::Inside);
        } else {
            p.rect_stroke(rect, CornerRadius::same(21), Stroke::new(1.0, LINE), StrokeKind::Inside);
        }
        p.galley(rect.center() - galley.size() / 2.0, galley, color);
        if resp.clicked() {
            self.acts.push(act);
        }
    }

    fn logo(&mut self, ui: &mut Ui) {
        let (rect, _) = ui.allocate_exact_size(vec2(34.0, 34.0), Sense::hover());
        self.svg(ui, &art::icon("leaf", None), rect);
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            ui.label(egui::RichText::new("jardim").font(serif(22.0)).color(GREEN));
            ui.label(egui::RichText::new("das descobertas").font(sans(11.0)).color(MUTED));
        });
    }

    fn header(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            self.logo(ui);
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                self.quiet_button(ui, "Área dos adultos", Act::Gate);
                ui.add_space(8.0);
                ui.label(egui::RichText::new(format!("★ {} descobertas", self.store.state.total())).font(sans(15.0)).color(EYEBROW));
            });
        });
    }

    fn eyebrow(ui: &mut Ui, text: &str) {
        ui.label(egui::RichText::new(text).font(sans(12.0)).color(EYEBROW).extra_letter_spacing(2.0));
    }

    // --------------------------------------------------------- tela inicial

    fn home(&mut self, ui: &mut Ui) {
        let width = ui.available_width();
        self.header(ui);
        ui.add_space(18.0);

        // Hero
        let wide = width > 760.0;
        let hero = |this: &mut Self, ui: &mut Ui| {
            ui.vertical(|ui| {
                Self::eyebrow(ui, "BRINCAR • PENSAR • DESCOBRIR");
                ui.add_space(6.0);
                ui.label(egui::RichText::new("Pequenas descobertas.").font(serif(40.0)));
                ui.label(egui::RichText::new("Grandes sorrisos.").font(serif(40.0)).color(EM));
                ui.add_space(8.0);
                ui.label(egui::RichText::new("Um jardim de ideias para aprender brincando, um passo de cada vez.").font(sans(17.0)).color(MUTED));
                ui.add_space(16.0);
                this.primary_button(ui, "Vamos explorar ↗", Act::Explore);
            });
        };
        if wide {
            ui.horizontal(|ui| {
                let left = (width - 470.0).max(300.0);
                ui.allocate_ui_with_layout(vec2(left, 270.0), Layout::top_down(Align::Min), |ui| {
                    ui.add_space(30.0);
                    hero(self, ui);
                });
                let (rect, _) = ui.allocate_exact_size(vec2(460.0, 265.0), Sense::hover());
                if ui.is_rect_visible(rect) {
                    self.svg(ui, &art::garden(), rect);
                }
            });
        } else {
            hero(self, ui);
            let w = width.min(460.0);
            let (rect, _) = ui.allocate_exact_size(vec2(w, w * 300.0 / 520.0), Sense::hover());
            self.svg(ui, &art::garden(), rect);
        }
        ui.add_space(28.0);

        // Seção de atividades
        let level = self.level();
        let available = self.catalog();
        if !available.iter().any(|g| g.category == self.category) {
            self.category = "all";
        }
        let top = ui.horizontal_wrapped(|ui| {
            ui.vertical(|ui| {
                Self::eyebrow(ui, "CADA IDADE, UMA DESCOBERTA");
                ui.label(egui::RichText::new("O que vamos brincar hoje?").font(serif(30.0)));
            });
            ui.add_space(20.0);
            for g in GROUPS.iter() {
                let sel = g.id == self.store.state.group;
                self.pill(ui, g.label, sel, Act::Age(g.id));
            }
        });
        if std::mem::take(&mut self.scroll_activities) {
            ui.scroll_to_rect(top.response.rect, Some(Align::Min));
        }
        ui.add_space(6.0);
        ui.label(
            egui::RichText::new(format!("{} • {} atividades gratuitas", GROUPS[level].name, available.len()))
                .font(sans(15.0))
                .color(MUTED),
        );
        ui.add_space(14.0);
        self.daily_overview(ui, &available);
        ui.add_space(16.0);

        ui.horizontal_wrapped(|ui| {
            for (id, label) in CATEGORIES.iter() {
                if *id == "all" || available.iter().any(|g| g.category == *id) {
                    self.tab(ui, label, *id == self.category, Act::Category(id));
                }
            }
        });
        ui.add_space(16.0);

        // Grade de cartões
        let gap = 16.0;
        let cols = ((width + gap) / (250.0 + gap)).floor().max(1.0) as usize;
        let card_w = (width - gap * (cols as f32 - 1.0)) / cols as f32;
        self.prewarm_home(ui.ctx(), card_w);
        let games: Vec<_> = available.into_iter().filter(|g| self.category == "all" || g.category == self.category).collect();
        for row in games.chunks(cols) {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = gap;
                for g in row {
                    self.game_card(ui, g, card_w);
                }
            });
            ui.add_space(gap);
        }
        ui.add_space(10.0);
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Feito para a curiosidade florescer.").font(sans(14.0)).color(MUTED));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(egui::RichText::new("Gratuito · Offline · Sem anúncios").font(sans(14.0)).color(MUTED));
            });
        });
        ui.add_space(20.0);
    }

    fn daily_overview(&mut self, ui: &mut Ui, available: &[&'static cur::Game]) {
        let total = available.len();
        let done = available.iter().filter(|g| self.store.state.daily(g.id).completed).count();
        let pct = if total > 0 { (done as f32 / total as f32 * 100.0).round() as u32 } else { 0 };
        egui::Frame::new()
            .fill(hex("#eaf0e2"))
            .stroke(Stroke::new(1.0, hex("#d4dfc7")))
            .corner_radius(22)
            .inner_margin(egui::Margin::symmetric(22, 20))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        Self::eyebrow(ui, "SEU JARDIM DE HOJE");
                        ui.label(egui::RichText::new(format!("{done} de {total} atividades feitas hoje")).font(serif(21.0)));
                        ui.label(egui::RichText::new("Novos desafios a cada dia. As marcações começam de novo diariamente.").font(sans(14.0)).color(MUTED));
                    });
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(egui::RichText::new(format!("{pct}% hoje")).font(sans(15.0)));
                        let (r, _) = ui.allocate_exact_size(vec2(180.0, 12.0), Sense::hover());
                        let p = ui.painter();
                        p.rect_filled(r, CornerRadius::same(6), Color32::WHITE);
                        let mut f = r;
                        f.set_width(r.width() * pct as f32 / 100.0);
                        if pct > 0 {
                            p.rect_filled(f, CornerRadius::same(6), GREEN);
                        }
                    });
                });
            });
    }

    fn game_card(&mut self, ui: &mut Ui, g: &'static cur::Game, w: f32) {
        let h = 290.0;
        let (rect, resp) = ui.allocate_exact_size(vec2(w, h), Sense::click());
        if resp.clicked() {
            self.acts.push(Act::Start(g.id));
        }
        // Culling: cartão fora da tela não gera vértices, draw calls nem texturas.
        if !ui.is_rect_visible(rect) {
            return;
        }
        let down = resp.is_pointer_button_down_on();
        let body = if down { rect.translate(vec2(0.0, 3.0)) } else { rect };
        let p = ui.painter().clone();
        let r = CornerRadius::same(22);
        p.rect_filled(rect.translate(vec2(0.0, 4.0)), r, hex("#e8e9de"));
        p.rect_filled(body, r, Color32::WHITE);
        let pic = Rect::from_min_size(body.min, vec2(w, 140.0));
        p.rect_filled(pic, CornerRadius { nw: 22, ne: 22, sw: 0, se: 0 }, card_bg(g.color));
        let (tw, th) = tile_size(g.icon);
        self.svg(ui, &art::tile(g.icon), Rect::from_center_size(pic.center(), vec2(tw, th)));
        p.text(pic.right_top() + vec2(-18.0, 16.0), Align2::RIGHT_TOP, "↗", sans(18.0), INK);
        p.rect_stroke(body, r, Stroke::new(1.0, hex("#e6e6d9")), StrokeKind::Inside);

        let mut y = pic.bottom() + 12.0;
        let x = body.left() + 16.0;
        let inner = w - 32.0;
        let title = p.layout(g.name.to_owned(), serif(19.0), INK, inner);
        y += title.size().y;
        p.galley(pos2(x, y - title.size().y), title, INK);
        let skill = p.layout(g.skill.to_owned(), sans(14.0), MUTED, inner);
        p.galley(pos2(x, y + 2.0), skill.clone(), MUTED);
        y += skill.size().y + 10.0;

        let d = self.store.state.daily(g.id);
        let prog = Rect::from_min_max(pos2(x - 6.0, y - 4.0), pos2(body.right() - 10.0, body.bottom() - 10.0));
        if d.completed {
            p.rect_filled(prog, CornerRadius::same(9), hex("#f1f7e9"));
        }
        let status = if d.completed { "✓ Feita hoje" } else if d.solved > 0 { "Em andamento hoje" } else { "Para descobrir hoje" };
        p.text(pos2(x, y), Align2::LEFT_TOP, status, serif(14.0), if d.completed { SUCCESS } else { INK });
        p.text(pos2(x, y + 20.0), Align2::LEFT_TOP, format!("{} de {GOAL} descobertas", d.solved), sans(13.0), MUTED);
        for i in 0..GOAL {
            let c = pos2(x + 6.0 + i as f32 * 16.0, y + 46.0);
            if i < d.solved {
                p.circle_filled(c, 5.0, GREEN);
            } else {
                p.circle_stroke(c, 5.0, Stroke::new(1.5, hex("#c9d3bf")));
            }
        }
    }

    // --------------------------------------------------------- tela do jogo

    fn game(&mut self, ui: &mut Ui, now: f64) {
        let Some(s) = &self.session else { return };
        let g = cur::game(s.game);
        let (round, length) = (s.round, s.length);
        let surface = hex(s.q.presentation.surface);
        let radius = s.q.presentation.radius as u8;

        ui.horizontal(|ui| {
            self.quiet_button(ui, "← Jardim", Act::Home);
            ui.add_space(12.0);
            self.logo(ui);
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let label = if self.store.state.voice { "♪ Som ligado" } else { "♪ Som desligado" };
                self.quiet_button(ui, label, Act::Voice);
            });
        });
        ui.add_space(12.0);

        egui::Frame::new().fill(surface).corner_radius(radius.saturating_add(8)).inner_margin(20).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(g.name).font(sans(15.0)).color(MUTED));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(egui::RichText::new(format!("Descoberta {} de {}", round + 1, length)).font(sans(15.0)).color(MUTED));
                });
            });
            // Pontinhos da rodada
            let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 18.0), Sense::hover());
            let start = r.center().x - (length as f32 - 1.0) * 11.0;
            for i in 0..length {
                let c = pos2(start + i as f32 * 22.0, r.center().y);
                if i < round {
                    ui.painter().circle_filled(c, 6.0, GREEN);
                } else if i == round {
                    ui.painter().circle_filled(c, 7.0, hex("#e9bd68"));
                } else {
                    ui.painter().circle_stroke(c, 6.0, Stroke::new(1.5, hex("#c9d3bf")));
                }
            }
            ui.add_space(8.0);
            let s = self.session.as_ref().unwrap();
            let prompt = s.q.prompt.clone();
            let memory = s.q.memory;
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    let w = ui.available_width();
                    let galley = ui.painter().layout(prompt.clone(), serif(30.0), INK, (w - 80.0).max(200.0));
                    let gw = galley.size().x + 64.0;
                    ui.add_space(((w - gw) / 2.0).max(0.0));
                    ui.label(egui::RichText::new(prompt).font(serif(30.0)));
                    ui.add_space(8.0);
                    let (rect, resp) = ui.allocate_exact_size(vec2(52.0, 52.0), Sense::click());
                    ui.painter().circle_filled(rect.center(), 26.0, Color32::WHITE);
                    ui.painter().circle_stroke(rect.center(), 26.0, Stroke::new(1.0, LINE));
                    ui.painter().text(rect.center(), Align2::CENTER_CENTER, "🔊", sans(22.0), GREEN);
                    if resp.clicked() {
                        self.acts.push(Act::Listen);
                    }
                });
                ui.label(
                    egui::RichText::new(if memory { "Toque nas cartas para descobrir os desenhos." } else { "Toque na resposta que você escolheu." })
                        .font(sans(15.0))
                        .color(MUTED),
                );
                ui.add_space(14.0);
                if memory {
                    self.memory_grid(ui, now);
                } else {
                    self.scene(ui);
                    ui.add_space(18.0);
                    self.choices(ui, now);
                }
            });
            ui.add_space(18.0);
            self.feedback(ui);
        });
    }

    fn feedback(&mut self, ui: &mut Ui) {
        let Some(s) = &self.session else { return };
        let (text, color, success) = match s.feedback {
            Feedback::Idle => ("Vamos descobrir juntos?".to_string(), INK, false),
            Feedback::Hint => (s.q.hint.clone(), INK, false),
            Feedback::Retry => (format!("Vamos tentar de novo? {}", s.q.hint), INK, false),
            Feedback::Success => ("★ Você descobriu! Muito bem.".to_string(), SUCCESS, true),
        };
        let last = s.round + 1 == s.length;
        let idle = matches!(s.feedback, Feedback::Idle);
        let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
        ui.painter().hline(r.x_range(), r.center().y, Stroke::new(1.0, LINE));
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            let w = ui.available_width() - 260.0;
            ui.allocate_ui_with_layout(vec2(w.max(200.0), 56.0), Layout::left_to_right(Align::Center), |ui| {
                ui.add(egui::Label::new(egui::RichText::new(text).font(if success { serif(18.0) } else { sans(17.0) }).color(color)).wrap());
            });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if success {
                    self.primary_button(ui, if last { "Ver meu jardim →" } else { "Próxima descoberta →" }, Act::Next);
                } else {
                    let label = if idle { "Uma dica" } else { "Ouvir dica" };
                    self.quiet_button(ui, label, Act::Hint);
                }
            });
        });
    }

    fn choices(&mut self, ui: &mut Ui, now: f64) {
        let Some(s) = &self.session else { return };
        let opts = s.q.options.clone();
        let answer = s.q.answer;
        let correct = s.correct;
        let wiggle = s.wiggle.filter(|(_, until)| *until > now);
        let radius = s.q.presentation.radius.min(40) as u8;
        let size = vec2(150.0, 140.0);
        let gap = 18.0;
        let w = ui.available_width();
        let per_row = ((w + gap) / (size.x + gap)).floor().max(1.0) as usize;
        for row in opts.chunks(per_row).enumerate() {
            let (ri, row) = row;
            ui.horizontal(|ui| {
                let row_w = row.len() as f32 * (size.x + gap) - gap;
                ui.add_space(((w - row_w) / 2.0).max(0.0));
                ui.spacing_mut().item_spacing.x = gap;
                for (ci, o) in row.iter().enumerate() {
                    let i = ri * per_row + ci;
                    let (rect, resp) = ui.allocate_exact_size(size, if correct { Sense::hover() } else { Sense::click() });
                    let mut body = rect;
                    if let Some((wi, until)) = wiggle {
                        if wi == i {
                            let t = (0.4 - (until - now)) as f32;
                            body = body.translate(vec2((t * 40.0).sin() * 6.0, 0.0));
                            ui.ctx().request_repaint();
                        }
                    }
                    let right = correct && i == answer;
                    let retry = wiggle.map(|(wi, _)| wi == i).unwrap_or(false);
                    let (fill, border, shadow) = if right {
                        (hex("#e1eed8"), hex("#78a36d"), hex("#78a36d"))
                    } else if retry {
                        (hex("#f8ebdb"), hex("#e5e7d9"), hex("#e3e6d6"))
                    } else {
                        (Color32::WHITE, hex("#e5e7d9"), hex("#e3e6d6"))
                    };
                    if resp.is_pointer_button_down_on() {
                        body = body.translate(vec2(0.0, 3.0));
                    }
                    let cr = CornerRadius::same(radius);
                    let p = ui.painter().clone();
                    p.rect_filled(rect.translate(vec2(0.0, 6.0)), cr, shadow);
                    p.rect_filled(body, cr, fill);
                    p.rect_stroke(body, cr, Stroke::new(2.0, border), StrokeKind::Inside);
                    self.visual(ui, &o.visual, body.shrink(13.0));
                    if resp.clicked() {
                        self.acts.push(Act::Answer(i));
                    }
                }
            });
            ui.add_space(gap);
        }
    }

    fn visual(&mut self, ui: &Ui, v: &Visual, rect: Rect) {
        let p = ui.painter().clone();
        match v {
            Visual::Number(n) => {
                p.text(rect.center(), Align2::CENTER_CENTER, n.to_string(), serif(52.0), INK);
            }
            Visual::Text(t) => {
                let size = if t.chars().count() > 3 { 28.0 } else { 46.0 };
                p.text(rect.center(), Align2::CENTER_CENTER, t, serif(size), INK);
            }
            Visual::Swatch(c) => {
                p.rect_filled(Rect::from_center_size(rect.center(), vec2(80.0, 80.0)), CornerRadius::same(18), hex(c));
            }
            Visual::Quantity(n) => {
                let (rot, color) = self.session.as_ref().map(|s| (s.q.presentation.rotation, s.q.presentation.item_color)).unwrap_or(([0; 24], "#e87961"));
                let apple = art::icon("apple", Some(color));
                let cols = (if *n <= 4 { 2 } else if *n <= 9 { 3 } else { 4 }).min(*n as usize).max(1);
                let rows = (*n as usize).div_ceil(cols);
                let cell = (rect.width() / cols as f32).min(rect.height() / rows as f32);
                let origin = rect.center() - vec2(cell * cols as f32, cell * rows as f32) / 2.0;
                for i in 0..*n as usize {
                    let c = origin + vec2((i % cols) as f32 * cell, (i / cols) as f32 * cell);
                    self.svg_rotated(ui, &apple, Rect::from_min_size(c, vec2(cell, cell)), rot[i % 24] as f32);
                }
            }
            Visual::Sized { size, .. } => {
                if let Some(svg) = visual_svg(v) {
                    let s = rect.width().min(rect.height()) * *size as f32;
                    self.svg(ui, &svg, Rect::from_center_size(rect.center(), vec2(s, s)));
                }
            }
            _ => {
                if let Some(svg) = visual_svg(v) {
                    let s = rect.width().min(rect.height());
                    self.svg(ui, &svg, Rect::from_center_size(rect.center(), vec2(s, s)));
                }
            }
        }
    }

    fn memory_grid(&mut self, ui: &mut Ui, now: f64) {
        let Some(s) = &mut self.session else { return };
        if let Some(t) = s.flip_back_at {
            if now >= t {
                s.revealed.clear();
                s.flip_back_at = None;
            } else {
                ui.ctx().request_repaint_after(std::time::Duration::from_secs_f64(t - now));
            }
        }
        let cards = s.q.cards.clone();
        let revealed = s.revealed.clone();
        let matched = s.matched.clone();
        let cols = if cards.len() <= 4 { 2 } else { 4 };
        let size = vec2(140.0, 140.0);
        let gap = 16.0;
        let w = ui.available_width();
        for (ri, row) in cards.chunks(cols).enumerate() {
            ui.horizontal(|ui| {
                let row_w = row.len() as f32 * (size.x + gap) - gap;
                ui.add_space(((w - row_w) / 2.0).max(0.0));
                ui.spacing_mut().item_spacing.x = gap;
                for (ci, c) in row.iter().enumerate() {
                    let i = ri * cols + ci;
                    let open = revealed.contains(&i) || matched.contains(&i);
                    let is_matched = matched.contains(&i);
                    let (rect, resp) = ui.allocate_exact_size(size, Sense::click());
                    let (fill, border, shadow) = if is_matched {
                        (hex("#eff5e9"), hex("#95b681"), hex("#95b681"))
                    } else if open {
                        (hex("#fffdf7"), hex("#d8cba5"), hex("#c3d2b4"))
                    } else {
                        (hex("#e2ead6"), hex("#c5d6b7"), hex("#c3d2b4"))
                    };
                    let cr = CornerRadius::same(20);
                    let p = ui.painter().clone();
                    p.rect_filled(rect.translate(vec2(0.0, 5.0)), cr, shadow);
                    p.rect_filled(rect, cr, fill);
                    p.rect_stroke(rect, cr, Stroke::new(2.0, border), StrokeKind::Inside);
                    let svg = if open { art::icon(c.kind, None) } else { art::icon("leaf", None) };
                    self.svg(ui, &svg, rect.shrink(22.0));
                    if resp.clicked() && !is_matched {
                        self.acts.push(Act::Flip(i));
                    }
                }
            });
            ui.add_space(gap);
        }
    }

    /// Item clicável de contagem (maçã).
    fn count_item(&mut self, ui: &mut Ui, i: usize, size: f32) {
        let Some(s) = &self.session else { return };
        let counted = s.counted.contains(&i);
        let deg = s.q.presentation.rotation[i % 24] as f32;
        let apple = art::icon("apple", Some(s.q.presentation.item_color));
        let (rect, resp) = ui.allocate_exact_size(vec2(size, size), Sense::click());
        if counted {
            ui.painter().circle_filled(rect.center(), size * 0.5, hex("#e0eddc"));
            ui.painter().circle_stroke(rect.center(), size * 0.5, Stroke::new(2.0, hex("#78a36d")));
        }
        self.svg_rotated(ui, &apple, rect.shrink(4.0), deg);
        if resp.clicked() {
            self.acts.push(Act::CountItem(i));
        }
    }

    fn items_row(&mut self, ui: &mut Ui, offset: usize, n: u32, size: f32) {
        let gap = self.session.as_ref().map(|s| s.q.presentation.item_gap as f32).unwrap_or(4.0);
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = vec2(gap, gap);
            for i in 0..n as usize {
                self.count_item(ui, offset + i, size);
            }
        });
    }

    fn centered_row(ui: &mut Ui, content_w: f32, add: impl FnOnce(&mut Ui)) {
        ui.horizontal(|ui| {
            ui.add_space(((ui.available_width() - content_w) / 2.0).max(0.0));
            add(ui);
        });
    }

    fn scene(&mut self, ui: &mut Ui) {
        let Some(scene) = self.session.as_ref().and_then(|s| s.q.scene.clone()) else { return };
        let box_ = |ui: &mut Ui, w: f32, h: f32| ui.allocate_exact_size(vec2(w, h), Sense::hover()).0;
        let centered = |ui: &mut Ui, w: f32, h: f32| {
            let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), h), Sense::hover());
            Rect::from_center_size(r.center(), vec2(w, h))
        };
        match scene {
            Scene::Color(c) => {
                let r = centered(ui, 120.0, 120.0);
                ui.painter().circle_filled(r.center(), 60.0, hex(&c));
            }
            Scene::Shape { shape, color } => {
                let r = centered(ui, 140.0, 140.0);
                self.svg(ui, &art::icon(shape, Some(&color)), r);
            }
            Scene::Items(n) => {
                let size = 64.0;
                let gap = 8.0;
                let w = (n as f32 * (size + gap)).min(ui.available_width());
                Self::centered_row(ui, w, |ui| {
                    ui.allocate_ui(vec2(w, 200.0), |ui| self.items_row(ui, 0, n, size));
                });
            }
            Scene::Sum(a, b) => {
                let size = 46.0;
                let gw = |n: u32| (n.min(5) as f32) * (size + 6.0);
                let w = gw(a) + gw(b) + 60.0;
                Self::centered_row(ui, w, |ui| {
                    ui.allocate_ui(vec2(gw(a), 200.0), |ui| self.items_row(ui, 0, a, size));
                    ui.label(egui::RichText::new("+").font(serif(40.0)));
                    ui.allocate_ui(vec2(gw(b), 200.0), |ui| self.items_row(ui, a as usize, b, size));
                });
            }
            Scene::Look => {
                ui.label(egui::RichText::new("Observe com atenção. Você consegue!").font(sans(16.0)).color(MUTED));
            }
            Scene::Sequence(items) => {
                let n = items.len() as f32 + 1.0;
                let r = centered(ui, n * 90.0, 84.0);
                for (i, s) in items.iter().enumerate() {
                    let cell = Rect::from_min_size(r.min + vec2(i as f32 * 90.0 + 3.0, 0.0), vec2(84.0, 84.0));
                    self.svg(ui, &art::icon(s, Some("#8b7bbd")), cell);
                }
                missing_box(ui, Rect::from_min_size(r.min + vec2(items.len() as f32 * 90.0 + 6.0, 6.0), vec2(72.0, 72.0)));
            }
            Scene::Numbers(items) => {
                let r = centered(ui, items.len() as f32 * 92.0, 120.0);
                for (i, v) in items.iter().enumerate() {
                    let cell = Rect::from_min_size(pos2(r.left() + i as f32 * 92.0, r.bottom() - 70.0 - i as f32 * 12.0), vec2(80.0, 70.0));
                    match v {
                        Some(n) => {
                            ui.painter().rect_filled(cell, CornerRadius::same(14), Color32::WHITE);
                            ui.painter().rect_stroke(cell, CornerRadius::same(14), Stroke::new(2.0, LINE), StrokeKind::Inside);
                            ui.painter().text(cell.center(), Align2::CENTER_CENTER, n.to_string(), serif(34.0), INK);
                        }
                        None => missing_box(ui, cell),
                    }
                }
            }
            Scene::ObjectModel(icon) => {
                let r = centered(ui, 150.0, 150.0);
                ui.painter().rect_filled(r, CornerRadius::same(24), Color32::WHITE);
                self.svg(ui, &art::icon(icon, None), r.shrink(14.0));
            }
            Scene::Sock { color, pattern } => {
                let r = centered(ui, 150.0, 150.0);
                self.svg(ui, &art::sock(color, pattern), r);
            }
            Scene::Tower(colors) => {
                let r = centered(ui, 150.0, 150.0);
                self.svg(ui, &art::tower(&colors), r);
            }
            Scene::Friends(n) => {
                let r = centered(ui, n as f32 * 80.0, 80.0);
                let bunny = art::icon("bunny", None);
                for i in 0..n {
                    self.svg(ui, &bunny, Rect::from_min_size(r.min + vec2(i as f32 * 80.0, 0.0), vec2(78.0, 78.0)));
                }
            }
            Scene::ColorSequence(colors) => {
                let r = centered(ui, (colors.len() + 1) as f32 * 84.0, 76.0);
                for (i, c) in colors.iter().enumerate() {
                    ui.painter().circle_filled(r.min + vec2(i as f32 * 84.0 + 38.0, 38.0), 34.0, hex(c));
                }
                missing_box(ui, Rect::from_min_size(r.min + vec2(colors.len() as f32 * 84.0 + 4.0, 4.0), vec2(68.0, 68.0)));
            }
            Scene::Letter(v) => {
                let r = centered(ui, 260.0, 120.0);
                ui.painter().rect_filled(r, CornerRadius::same(24), Color32::WHITE);
                ui.painter().text(r.center(), Align2::CENTER_CENTER, v, serif(64.0), INK);
            }
            Scene::WordPicture { word, icon } => {
                let w = if word.is_empty() { 140.0 } else { 360.0 };
                let r = centered(ui, w, 130.0);
                self.svg(ui, &art::icon(icon, None), Rect::from_min_size(r.min, vec2(130.0, 130.0)));
                if !word.is_empty() {
                    ui.painter().text(pos2(r.min.x + 150.0, r.center().y), Align2::LEFT_CENTER, word, serif(46.0), INK);
                }
            }
            Scene::Subtract { a, b } => {
                let size = 52.0;
                let r = centered(ui, a as f32 * (size + 6.0), size);
                let apple = art::icon("apple", None);
                for i in 0..a {
                    let cell = Rect::from_min_size(r.min + vec2(i as f32 * (size + 6.0), 0.0), vec2(size, size));
                    self.svg(ui, &apple, cell);
                    if i >= a - b {
                        ui.painter().rect_filled(cell, CornerRadius::same(10), Color32::from_rgba_unmultiplied(248, 246, 238, 150));
                        let s = Stroke::new(4.0, hex("#c95a46"));
                        ui.painter().line_segment([cell.left_top() + vec2(6.0, 6.0), cell.right_bottom() - vec2(6.0, 6.0)], s);
                    }
                }
            }
            Scene::Equation { a, total } => {
                let r = centered(ui, 5.0 * 80.0, 76.0);
                let cells = [Some(a.to_string()), Some("+".into()), None, Some("=".into()), Some(total.to_string())];
                for (i, c) in cells.iter().enumerate() {
                    let cell = Rect::from_min_size(r.min + vec2(i as f32 * 80.0, 0.0), vec2(72.0, 72.0));
                    match c {
                        Some(t) => {
                            ui.painter().text(cell.center(), Align2::CENTER_CENTER, t, serif(40.0), INK);
                        }
                        None => missing_box(ui, cell),
                    }
                }
            }
            Scene::TenFrame(n) => {
                let cell = 56.0;
                let r = centered(ui, 5.0 * cell + 12.0, 2.0 * cell + 12.0);
                ui.painter().rect_filled(r, CornerRadius::same(12), Color32::WHITE);
                ui.painter().rect_stroke(r, CornerRadius::same(12), Stroke::new(2.0, hex("#92b4ad")), StrokeKind::Inside);
                for i in 0..10u32 {
                    let c = r.min + vec2(6.0 + (i % 5) as f32 * cell, 6.0 + (i / 5) as f32 * cell);
                    let cr = Rect::from_min_size(c, vec2(cell, cell));
                    ui.painter().rect_stroke(cr, CornerRadius::ZERO, Stroke::new(1.0, LINE), StrokeKind::Inside);
                    if i < n {
                        ui.painter().circle_filled(cr.center(), cell * 0.34, hex("#e87961"));
                    }
                }
            }
            Scene::Tens { tens, units } => {
                let sq = 13.0;
                let w = tens as f32 * (sq + 14.0) + 30.0 + units as f32 * (sq + 4.0);
                let r = centered(ui, w, 10.0 * sq + 4.0);
                for t in 0..tens {
                    for k in 0..10 {
                        let c = r.min + vec2(t as f32 * (sq + 14.0), k as f32 * sq);
                        let q = Rect::from_min_size(c, vec2(sq, sq)).shrink(0.5);
                        ui.painter().rect_filled(q, CornerRadius::same(2), hex("#5799be"));
                    }
                }
                let ux = r.min.x + tens as f32 * (sq + 14.0) + 30.0;
                for u in 0..units {
                    let q = Rect::from_min_size(pos2(ux + u as f32 * (sq + 4.0), r.bottom() - sq), vec2(sq, sq));
                    ui.painter().rect_filled(q, CornerRadius::same(2), hex("#e0b849"));
                }
            }
            Scene::Clock(h) => {
                let r = centered(ui, 170.0, 170.0);
                self.svg(ui, &art::clock(h), r);
            }
            Scene::Story { text, icon } => {
                let w = ui.available_width().min(720.0);
                Self::centered_row(ui, w, |ui| {
                    let r = box_(ui, 96.0, 96.0);
                    self.svg(ui, &art::icon(icon, None), r);
                    ui.add_space(12.0);
                    ui.allocate_ui(vec2(w - 110.0, 110.0), |ui| {
                        ui.add(egui::Label::new(egui::RichText::new(text).font(sans(19.0))).wrap());
                    });
                });
            }
        }
    }

    // --------------------------------------------------------- tela final

    fn finish_screen(&mut self, ui: &mut Ui) {
        let Some(s) = &self.session else { return };
        let len = s.length;
        let d = self.store.state.daily(s.game);
        self.header(ui);
        ui.add_space(30.0);
        ui.vertical_centered(|ui| {
            let (r, _) = ui.allocate_exact_size(vec2(130.0, 130.0), Sense::hover());
            self.svg(ui, &art::icon("flower", None), r);
            Self::eyebrow(ui, "A CURIOSIDADE FLORESCEU!");
            let headline = if len == 1 { "Uma nova descoberta!".to_string() } else { format!("{len} novas descobertas!") };
            ui.label(egui::RichText::new(headline).font(serif(40.0)));
            ui.label(
                egui::RichText::new(if d.completed { "✓ Atividade feita hoje!".to_string() } else { format!("{} de {GOAL} descobertas hoje", d.solved) })
                    .font(serif(18.0))
                    .color(SUCCESS),
            );
            ui.add_space(8.0);
            ui.label(egui::RichText::new("Você observou, tentou e aprendeu.").font(sans(17.0)).color(MUTED));
            ui.label(egui::RichText::new("Que tal alongar o corpo e olhar o mundo lá fora?").font(sans(17.0)).color(MUTED));
            ui.label(egui::RichText::new("★ ".repeat(len as usize)).font(sans(22.0)).color(hex("#e9bd68")));
            ui.add_space(14.0);
            self.primary_button(ui, "Voltar para o jardim", Act::Home);
            ui.add_space(10.0);
            ui.label(egui::RichText::new("Cada tentativa faz parte de aprender.").font(sans(14.0)).color(MUTED));
        });
    }

    // --------------------------------------------------------- modais

    fn modal(&mut self, ctx: &egui::Context) {
        if matches!(self.modal, Modal::None) {
            return;
        }
        // Fundo escurecido simples — sem `backdrop-filter: blur`, que era o
        // efeito mais caro para a GPU Mali no app antigo.
        let screen = ctx.screen_rect();
        egui::Area::new("overlay".into()).order(egui::Order::Foreground).fixed_pos(Pos2::ZERO).show(ctx, |ui| {
            let resp = ui.allocate_rect(screen, Sense::click());
            ui.painter().rect_filled(screen, CornerRadius::ZERO, Color32::from_rgba_unmultiplied(0x23, 0x3b, 0x36, 0xa8));
            if resp.clicked() {
                // Toque fora não fecha (igual ao original).
            }
        });
        let w = (screen.width() - 40.0).min(560.0);
        egui::Area::new("dialog".into()).order(egui::Order::Tooltip).anchor(Align2::CENTER_CENTER, Vec2::ZERO).show(ctx, |ui| {
            egui::Frame::new().fill(PAPER).corner_radius(27).inner_margin(30).show(ui, |ui| {
                ui.set_width(w - 60.0);
                egui::ScrollArea::vertical().id_salt("modal").max_height(screen.height() * 0.85 - 60.0).show(ui, |ui| match &self.modal {
                    Modal::Gate { .. } => self.gate_ui(ui),
                    Modal::Parents => self.parents_ui(ui),
                    Modal::ResetConfirm => {
                        ui.label(egui::RichText::new("Apagar todas as descobertas?").font(serif(26.0)));
                        ui.label(egui::RichText::new("Isso apaga o progresso das três faixas etárias neste dispositivo.").font(sans(16.0)).color(MUTED));
                        ui.add_space(16.0);
                        ui.horizontal(|ui| {
                            self.quiet_button(ui, "Cancelar", Act::Parents);
                            ui.add_space(10.0);
                            self.danger_button(ui, "Sim, apagar", Act::Reset);
                        });
                    }
                    Modal::None => {}
                });
            });
        });
    }

    fn danger_button(&mut self, ui: &mut Ui, text: &str, act: Act) {
        let color = hex("#9c513c");
        let galley = ui.painter().layout_no_wrap(text.to_owned(), serif(16.0), color);
        let (rect, resp) = ui.allocate_exact_size(vec2(galley.size().x + 32.0, 48.0), Sense::click());
        ui.painter().rect_filled(rect, CornerRadius::same(15), hex("#f2e2d8"));
        ui.painter().galley(rect.center() - galley.size() / 2.0, galley, color);
        if resp.clicked() {
            self.acts.push(act);
        }
    }

    fn gate_ui(&mut self, ui: &mut Ui) {
        let Modal::Gate { a, b, input, error } = &self.modal else { return };
        let (a, b, input, error) = (*a, *b, input.clone(), *error);
        Self::eyebrow(ui, "UM MOMENTO PARA OS ADULTOS");
        ui.label(egui::RichText::new("Vamos cuidar do jardim?").font(serif(26.0)));
        ui.label(egui::RichText::new("Para abrir as configurações, resolva:").font(sans(16.0)).color(MUTED));
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(format!("Quanto é {a} × {b}?")).font(serif(22.0)));
            ui.add_space(12.0);
            let (r, _) = ui.allocate_exact_size(vec2(110.0, 50.0), Sense::hover());
            ui.painter().rect_filled(r, CornerRadius::same(12), Color32::WHITE);
            ui.painter().rect_stroke(r, CornerRadius::same(12), Stroke::new(2.0, LINE), StrokeKind::Inside);
            ui.painter().text(r.center(), Align2::CENTER_CENTER, &input, serif(26.0), INK);
        });
        if error {
            ui.label(egui::RichText::new("Tente mais uma vez.").font(sans(15.0)).color(hex("#9c513c")));
        }
        ui.add_space(10.0);
        // Teclado numérico próprio: no tablet é mais rápido que abrir o teclado do sistema.
        for row in ["123", "456", "789", "<0"] {
            ui.horizontal(|ui| {
                for c in row.chars() {
                    let label = if c == '<' { "⌫".to_string() } else { c.to_string() };
                    let (r, resp) = ui.allocate_exact_size(vec2(72.0, 52.0), Sense::click());
                    ui.painter().rect_filled(r, CornerRadius::same(12), if resp.is_pointer_button_down_on() { LINE } else { QUIET });
                    ui.painter().text(r.center(), Align2::CENTER_CENTER, label, serif(22.0), INK);
                    if resp.clicked() {
                        self.acts.push(Act::GateKey(c));
                    }
                }
            });
        }
        ui.add_space(14.0);
        ui.horizontal(|ui| {
            self.quiet_button(ui, "Voltar", Act::Close);
            ui.add_space(10.0);
            self.primary_button(ui, "Entrar", Act::GateSubmit);
        });
    }

    fn parents_ui(&mut self, ui: &mut Ui) {
        Self::eyebrow(ui, "ÁREA DOS ADULTOS");
        ui.label(egui::RichText::new("Aprender no próprio ritmo").font(serif(26.0)));
        ui.add(egui::Label::new(egui::RichText::new("As faixas são sugestões. Escolha a que combina com a criança e acompanhe as atividades, especialmente aos 2–3 anos.").font(sans(15.0)).color(MUTED)).wrap());
        ui.add_space(10.0);
        ui.label(egui::RichText::new("Faixa etária").font(serif(18.0)));
        ui.horizontal(|ui| {
            for g in GROUPS.iter() {
                let sel = g.id == self.store.state.group;
                self.pill(ui, g.label, sel, Act::ParentAge(g.id));
            }
        });
        ui.add_space(10.0);
        ui.label(egui::RichText::new("Narração em português").font(serif(18.0)));
        let v = if self.store.state.voice { "Ligada — toque para desligar" } else { "Desligada — toque para ligar" };
        self.quiet_button(ui, v, Act::ParentVoice);
        ui.add(egui::Label::new(egui::RichText::new("Usa a voz do Android. O primeiro uso pode exigir uma voz pt-BR já instalada. As atividades funcionam sem áudio.").font(sans(13.0)).color(MUTED)).wrap());
        ui.add_space(10.0);
        let label = GROUPS[self.level()].label;
        ui.label(egui::RichText::new(format!("Descobertas por atividade · {label}")).font(serif(18.0)));
        for g in cur::GAMES.iter() {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(g.name).font(sans(14.0)));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let d = self.store.state.daily(g.id).solved;
                    let t = self.store.state.stat(g.id).solved;
                    ui.label(egui::RichText::new(format!("{d}/5 hoje · {t} no total")).font(sans(14.0)).color(MUTED));
                });
            });
        }
        ui.add_space(8.0);
        ui.add(egui::Label::new(egui::RichText::new("Os dados ficam somente neste dispositivo. Sem conta, anúncios, compras ou acesso à internet. A pergunta de entrada é uma barreira simples para crianças pequenas.").font(sans(13.0)).color(MUTED)).wrap());
        ui.add_space(14.0);
        ui.horizontal(|ui| {
            self.danger_button(ui, "Apagar progresso", Act::ResetConfirm);
            ui.add_space(10.0);
            self.primary_button(ui, "Pronto", Act::Home);
        });
    }
}

fn missing_box(ui: &Ui, r: Rect) {
    let p = ui.painter();
    p.rect_filled(r, CornerRadius::same(14), hex("#fffdf8"));
    p.rect_stroke(r, CornerRadius::same(14), Stroke::new(2.5, hex("#e9bd68")), StrokeKind::Inside);
    p.text(r.center(), Align2::CENTER_CENTER, "?", serif(34.0), hex("#c8963c"));
}

const CHOICE_ART: f32 = 114.0;

fn tile_size(icon: &str) -> (f32, f32) {
    const SINGLE: [&str; 9] = ["flower", "letters", "book", "clock", "socks", "cup", "tower", "container", "colorpath"];
    if SINGLE.contains(&icon) {
        (110.0, 110.0)
    } else {
        (180.0, 90.0)
    }
}

pub fn visual_svg(v: &Visual) -> Option<String> {
    Some(match v {
        Visual::Icon { kind, color } => art::icon(kind, color.as_deref()),
        Visual::Sized { shape, color, .. } => art::icon(shape, Some(color)),
        Visual::Position(d) => art::position(d),
        Visual::Sock { color, pattern } => art::sock(color, pattern),
        Visual::Cup(a) => art::cup(*a),
        Visual::Container(p) => art::container(p),
        Visual::Tower(c) => art::tower(c),
        _ => return None,
    })
}

fn install_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let (sans_paths, serif_paths) = crate::raster::font_paths();
    let load = |paths: &[&str]| paths.iter().find_map(|p| std::fs::read(p).ok());
    if let Some(b) = load(sans_paths) {
        fonts.font_data.insert("sys-sans".into(), std::sync::Arc::new(egui::FontData::from_owned(b)));
        fonts.families.entry(FontFamily::Proportional).or_default().insert(0, "sys-sans".into());
    }
    let mut serif_family = vec![];
    if let Some(b) = load(serif_paths) {
        fonts.font_data.insert("sys-serif".into(), std::sync::Arc::new(egui::FontData::from_owned(b)));
        serif_family.push("sys-serif".to_string());
    }
    // Reserva para setas e símbolos que a Roboto não tem (←, ✓, ⌫ …).
    let mut extra = vec![];
    for (i, p) in [
        "/system/fonts/NotoSerif-Regular.ttf",
        "/system/fonts/NotoSansSymbols-Regular-Subsetted.ttf",
        "/system/fonts/NotoSansSymbols-Regular-Subsetted2.ttf",
    ]
    .iter()
    .enumerate()
    {
        if let Ok(b) = std::fs::read(p) {
            let name = format!("sys-fallback-{i}");
            fonts.font_data.insert(name.clone(), std::sync::Arc::new(egui::FontData::from_owned(b)));
            extra.push(name);
        }
    }
    if let Some(f) = fonts.families.get_mut(&FontFamily::Proportional) {
        let at = f.iter().position(|n| n == "sys-sans").map(|i| i + 1).unwrap_or(0);
        for (k, n) in extra.into_iter().enumerate() {
            f.insert(at + k, n);
        }
    }
    // Demais fallbacks (emoji) vêm das fontes padrão do egui.
    serif_family.extend(fonts.families.get(&FontFamily::Proportional).cloned().unwrap_or_default());
    fonts.families.insert(FontFamily::Name("serif".into()), serif_family);
    ctx.set_fonts(fonts);
}

impl eframe::App for JardimApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let now = ctx.input(|i| i.time);
        if std::env::var_os("JARDIM_TRACE").is_some() || cfg!(feature = "trace") {
            // cpu_usage = update + tessellação + paint do frame anterior.
            let (ev, touch) = ctx.input(|i| {
                let t = i.events.iter().filter(|e| matches!(e, egui::Event::Touch { .. } | egui::Event::PointerMoved(_))).count();
                (i.events.len(), t)
            });
            log::info!(
                "frame dt={:.1}ms cpu={:.1}ms ev={ev} touch={touch}",
                ctx.input(|i| i.unstable_dt) * 1000.0,
                frame.info().cpu_usage.unwrap_or(0.0) * 1000.0
            );
        }

        // Ciclo de vida: perdeu o foco = app em segundo plano.
        let focused = ctx.input(|i| i.viewport().focused).unwrap_or(true);
        if !focused && !self.hidden {
            self.hidden = true;
            self.stop_voice();
        } else if focused && self.hidden {
            self.hidden = false;
            self.entry_seed = rng::entry_seed();
            if self.store.state.ensure_day() {
                self.store.save();
            }
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.back(ctx);
        }
        if self.screen == Screen::Game {
            self.prewarm_question(ctx);
        }

        // Área segura: a janela nativa cobre barra de status e taskbar.
        let ppp = ctx.pixels_per_point();
        let full = ctx.screen_rect();
        let cr = self.android.content_rect();
        let (top, bottom) = if cr.bottom > cr.top {
            (cr.top as f32 / ppp, (full.height() - cr.bottom as f32 / ppp).max(0.0))
        } else {
            (0.0, 0.0)
        };
        let screen = self.screen;
        let margin = egui::Margin { left: 24, right: 24, top: (16.0 + top) as i8, bottom: (8.0 + bottom) as i8 };
        egui::CentralPanel::default().frame(egui::Frame::new().fill(PAPER).inner_margin(margin)).show(ctx, |ui| {
            let mut area = egui::ScrollArea::vertical().id_salt(screen as u8).auto_shrink([false, false]);
            if std::mem::take(&mut self.scroll_top) {
                area = area.vertical_scroll_offset(0.0);
            }
            area.show(ui, |ui| {
                let w = ui.available_width().min(1180.0);
                ui.vertical_centered(|ui| {
                    ui.set_max_width(w);
                    ui.with_layout(Layout::top_down(Align::Min), |ui| match screen {
                        Screen::Home => self.home(ui),
                        Screen::Game => self.game(ui, now),
                        Screen::Finish => self.finish_screen(ui),
                    });
                });
            });
        });
        self.modal(ctx);

        if let Some((text, until)) = &self.toast {
            if now > *until {
                self.toast = None;
            } else {
                let t = text.clone();
                egui::Area::new("toast".into()).anchor(Align2::CENTER_BOTTOM, vec2(0.0, -30.0)).show(ctx, |ui| {
                    egui::Frame::new().fill(INK).corner_radius(14).inner_margin(14).show(ui, |ui| {
                        ui.label(egui::RichText::new(t).color(Color32::WHITE));
                    });
                });
            }
        }

        for act in std::mem::take(&mut self.acts) {
            self.apply(act, now);
            ctx.request_repaint();
        }

        // Acorda sozinho na meia-noite para virar o dia (como refreshDailyCalendar).
        let secs = {
            use chrono::{Local, Timelike};
            let n = Local::now();
            86_400 - n.num_seconds_from_midnight() as u64 + 1
        };
        ctx.request_repaint_after(std::time::Duration::from_secs(secs));
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        PAPER.to_normalized_gamma_f32()
    }
}
