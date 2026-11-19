//! Geradores determinísticos — cópia bit a bit de `curriculum.js`/`variation.js`
//! (mulberry32 + FNV-1a), para que a mesma semente gere a mesma atividade.

/// mulberry32, igual a `Curriculum.random(seed)`.
pub struct Rng(u32);

impl Rng {
    pub fn new(seed: u32) -> Self {
        Rng(seed)
    }

    /// Número em [0, 1), como `rng()` no JS.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_add(0x6d2b_79f5);
        let a = self.0;
        let mut t = (a ^ (a >> 15)).wrapping_mul(1 | a);
        t = (t.wrapping_add((t ^ (t >> 7)).wrapping_mul(61 | t))) ^ t;
        f64::from(t ^ (t >> 14)) / 4_294_967_296.0
    }

    /// `Math.floor(rng() * n)`
    pub fn below(&mut self, n: usize) -> usize {
        (self.next() * n as f64) as usize
    }

    pub fn pick<T: Clone>(&mut self, xs: &[T]) -> T {
        xs[self.below(xs.len())].clone()
    }

    /// `lo + Math.floor(rng() * (hi - lo + 1))`
    pub fn int(&mut self, lo: i32, hi: i32) -> i32 {
        lo + (self.next() * f64::from(hi - lo + 1)) as i32
    }

    /// Fisher–Yates igual ao `shuffle` do JS (devolve cópia).
    pub fn shuffle<T: Clone>(&mut self, xs: &[T]) -> Vec<T> {
        let mut a = xs.to_vec();
        for i in (1..a.len()).rev() {
            let j = (self.next() * (i + 1) as f64) as usize;
            a.swap(i, j);
        }
        a
    }
}

/// FNV-1a de 32 bits, igual a `Variation.hash`.
pub fn hash(text: &str) -> u32 {
    let mut v: u32 = 2_166_136_261;
    for ch in text.chars() {
        // JS: ch.charCodeAt(0) → primeira unidade UTF-16 do code point.
        let mut buf = [0u16; 2];
        v ^= u32::from(ch.encode_utf16(&mut buf)[0]);
        v = v.wrapping_mul(16_777_619);
    }
    v
}

pub fn seed_for(day: &str, group: &str, game: &str, round: u64, entry: u32) -> u32 {
    hash(&format!("{day}|{group}|{game}|{round}|{entry}"))
}

pub fn entry_seed() -> u32 {
    let mut b = [0u8; 4];
    if getrandom::getrandom(&mut b).is_ok() {
        u32::from_ne_bytes(b)
    } else {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u32)
            .unwrap_or(0x1234_5678)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_js_reference() {
        // Valores obtidos rodando o curriculum.js/variation.js originais no Node
        // (tests/gen_reference.js).
        let mut r = Rng::new(42);
        assert_eq!(r.next(), 0.6011037519201636);
        assert_eq!(r.next(), 0.44829055899754167);
        assert_eq!(r.next(), 0.8524657934904099);
        assert_eq!(hash("a|b"), 692_878_806);
        assert_eq!(seed_for("2026-11-19", "little", "colors", 1, 7), 987_461_503);
    }
}
