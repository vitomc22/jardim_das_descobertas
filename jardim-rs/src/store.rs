//! Estado salvo (equivale a `localStorage["jardim-v1"]` + progress.js).
//! A gravação acontece numa thread separada: a UI só serializa e envia.

use crate::curriculum::{GAMES, GROUPS};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};

pub const GOAL: u32 = 5;

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub struct Stat {
    pub solved: u64,
    pub tries: u64,
    pub sets: u64,
    pub rounds: u64,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default)]
pub struct DailyRec {
    pub solved: u32,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Daily {
    pub date: String,
    pub activities: BTreeMap<String, DailyRec>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct State {
    pub version: u32,
    pub group: String,
    pub voice: bool,
    pub stats: BTreeMap<String, Stat>,
    pub daily: Daily,
}

pub fn local_day() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

fn valid_key(k: &str) -> bool {
    let Some((age, game)) = k.split_once(':') else { return false };
    GROUPS.iter().any(|g| g.id == age) && GAMES.iter().any(|g| g.id == game)
}

impl State {
    fn fresh() -> Self {
        State {
            version: 1,
            group: "little".into(),
            voice: true,
            stats: BTreeMap::new(),
            daily: Daily { date: local_day(), activities: BTreeMap::new() },
        }
    }

    /// Mesma validação do `load()` + `DailyProgress.normalise` do JS.
    fn sanitize(mut self) -> Self {
        if self.version != 1 || !GROUPS.iter().any(|g| g.id == self.group) {
            return Self::fresh();
        }
        self.stats.retain(|k, _| valid_key(k));
        let today = local_day();
        if self.daily.date != today {
            self.daily = Daily { date: today, activities: BTreeMap::new() };
        }
        self.daily.activities.retain(|k, _| valid_key(k));
        for r in self.daily.activities.values_mut() {
            r.solved = r.solved.min(GOAL);
            r.completed = r.solved == GOAL;
        }
        self
    }

    /// Vira o dia se necessário. Devolve true se mudou.
    pub fn ensure_day(&mut self) -> bool {
        let today = local_day();
        if self.daily.date == today {
            return false;
        }
        self.daily = Daily { date: today, activities: BTreeMap::new() };
        true
    }

    fn key(&self, game: &str) -> String {
        format!("{}:{game}", self.group)
    }
    pub fn stat(&self, game: &str) -> Stat {
        self.stats.get(&self.key(game)).copied().unwrap_or_default()
    }
    pub fn stat_mut(&mut self, game: &str) -> &mut Stat {
        let k = self.key(game);
        self.stats.entry(k).or_default()
    }
    pub fn daily(&self, game: &str) -> DailyRec {
        self.daily.activities.get(&self.key(game)).copied().unwrap_or_default()
    }
    pub fn record_daily(&mut self, game: &str) {
        let k = self.key(game);
        let r = self.daily.activities.entry(k).or_default();
        r.solved = (r.solved + 1).min(GOAL);
        r.completed = r.solved == GOAL;
    }
    pub fn total(&self) -> u64 {
        self.stats.values().map(|s| s.solved).sum()
    }
}

pub struct Store {
    pub state: State,
    tx: Option<Sender<Vec<u8>>>,
}

impl Store {
    pub fn open(dir: Option<PathBuf>) -> Self {
        let path = dir.map(|d| d.join("jardim-v1.json"));
        let state = path
            .as_ref()
            .and_then(|p| std::fs::read(p).ok())
            .and_then(|b| serde_json::from_slice::<State>(&b).ok())
            .map(State::sanitize)
            .unwrap_or_else(State::fresh);
        let tx = path.map(|path| {
            let (tx, rx) = channel::<Vec<u8>>();
            std::thread::Builder::new()
                .name("jardim-save".into())
                .spawn(move || {
                    while let Ok(mut bytes) = rx.recv() {
                        // Junta rajadas de gravações: só a última importa.
                        while let Ok(newer) = rx.try_recv() {
                            bytes = newer;
                        }
                        let tmp = path.with_extension("tmp");
                        if std::fs::write(&tmp, &bytes).is_ok() {
                            let _ = std::fs::rename(&tmp, &path);
                        }
                    }
                })
                .ok();
            tx
        });
        Store { state, tx }
    }

    pub fn save(&self) {
        if let (Some(tx), Ok(bytes)) = (&self.tx, serde_json::to_vec(&self.state)) {
            let _ = tx.send(bytes);
        }
    }
}
