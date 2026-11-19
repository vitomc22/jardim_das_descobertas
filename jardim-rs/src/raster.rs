//! SVG → textura de GPU, com cache.
//!
//! * Cada (SVG, tamanho em px) é rasterizado uma única vez (resvg/tiny-skia,
//!   que usa NEON no aarch64) e fica guardado como textura.
//! * `prewarm` rasteriza vários desenhos em paralelo nos 4 núcleos (rayon).
//! * O buffer RGBA pré-multiplicado do tiny-skia é reinterpretado como
//!   `Vec<Color32>` sem cópia (`bytemuck::cast_vec`): mesmo layout de memória.

use egui::{Color32, ColorImage, Context, TextureHandle, TextureOptions};
use rayon::prelude::*;
use resvg::{tiny_skia, usvg};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

pub struct Raster {
    opts: usvg::Options<'static>,
    cache: HashMap<(u64, u32, u32), TextureHandle>,
}

/// Tamanhos arredondados para cima em múltiplos de 8 px: reaproveita texturas
/// entre telas com tamanhos quase iguais.
fn bucket(px: f32) -> u32 {
    ((px.max(8.0) / 8.0).ceil() as u32) * 8
}

fn key(svg: &str, w: u32, h: u32) -> (u64, u32, u32) {
    let mut s = std::collections::hash_map::DefaultHasher::new();
    svg.hash(&mut s);
    (s.finish(), w, h)
}

pub fn font_paths() -> (&'static [&'static str], &'static [&'static str]) {
    // (sans, serif-bold) — fontes do próprio sistema, nada embutido no APK.
    if cfg!(target_os = "android") {
        (&["/system/fonts/RobotoStatic-Regular.ttf", "/system/fonts/Roboto-Regular.ttf"], &["/system/fonts/NotoSerif-Bold.ttf"])
    } else {
        (
            &["/usr/share/fonts/liberation-sans-fonts/LiberationSans-Regular.ttf"],
            &["/usr/share/fonts/liberation-serif-fonts/LiberationSerif-Bold.ttf"],
        )
    }
}

fn rasterize(opts: &usvg::Options, svg: &str, w: u32, h: u32) -> Option<ColorImage> {
    let tree = usvg::Tree::from_str(svg, opts).ok()?;
    let mut pixmap = tiny_skia::Pixmap::new(w, h)?;
    let size = tree.size();
    let t = tiny_skia::Transform::from_scale(w as f32 / size.width(), h as f32 / size.height());
    resvg::render(&tree, t, &mut pixmap.as_mut());
    // Zero-copy: Vec<u8> RGBA premultiplicado → Vec<Color32> (mesmo layout).
    let pixels: Vec<Color32> = bytemuck::cast_vec(pixmap.take());
    Some(ColorImage { size: [w as usize, h as usize], pixels })
}

impl Raster {
    pub fn new() -> Self {
        let mut db = usvg::fontdb::Database::new();
        let (sans, serif) = font_paths();
        // Carrega só os dois arquivos necessários (varrer /system/fonts inteiro
        // custaria centenas de ms na abertura).
        if let Some(b) = sans.iter().find_map(|p| std::fs::read(p).ok()) {
            db.load_font_data(b);
        }
        if let Some(b) = serif.iter().find_map(|p| std::fs::read(p).ok()) {
            db.load_font_data(b);
        }
        let fams: Vec<String> = db.faces().filter_map(|f| f.families.first().map(|(n, _)| n.clone())).collect();
        if let Some(f) = fams.first() {
            db.set_sans_serif_family(f.clone());
        }
        if let Some(f) = fams.last() {
            db.set_serif_family(f.clone());
        }
        let opts = usvg::Options { fontdb: Arc::new(db), ..Default::default() };
        Raster { opts, cache: HashMap::new() }
    }

    /// Textura para o SVG no tamanho (em pixels físicos) pedido.
    pub fn get(&mut self, ctx: &Context, svg: &str, w_px: f32, h_px: f32) -> TextureHandle {
        let (w, h) = (bucket(w_px), bucket(h_px));
        let k = key(svg, w, h);
        if let Some(t) = self.cache.get(&k) {
            return t.clone();
        }
        let img = rasterize(&self.opts, svg, w, h).unwrap_or_else(|| ColorImage::new([1, 1], Color32::TRANSPARENT));
        let tex = ctx.load_texture(format!("svg{:x}", k.0), img, TextureOptions::LINEAR);
        self.cache.insert(k, tex.clone());
        tex
    }

    /// Rasteriza em paralelo tudo o que ainda não está no cache.
    pub fn prewarm(&mut self, ctx: &Context, jobs: &[(String, f32, f32)]) {
        let todo: Vec<(String, u32, u32)> = jobs
            .iter()
            .map(|(s, w, h)| (s.clone(), bucket(*w), bucket(*h)))
            .filter(|(s, w, h)| !self.cache.contains_key(&key(s, *w, *h)))
            .collect();
        if todo.is_empty() {
            return;
        }
        let opts = &self.opts;
        let images: Vec<_> = todo
            .par_iter()
            .map(|(s, w, h)| (key(s, *w, *h), rasterize(opts, s, *w, *h)))
            .collect();
        for (k, img) in images {
            if let Some(img) = img {
                let tex = ctx.load_texture(format!("svg{:x}", k.0), img, TextureOptions::LINEAR);
                self.cache.insert(k, tex);
            }
        }
    }

    /// Libera texturas quando o cache passa do limite (ex.: muitas perguntas).
    pub fn trim(&mut self, max: usize) {
        if self.cache.len() > max {
            self.cache.clear();
        }
    }
}
