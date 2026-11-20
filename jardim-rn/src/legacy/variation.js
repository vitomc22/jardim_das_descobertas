/* Task-aware visual variation: irrelevant cues vary; answer rules remain intact. MIT. */
(function (root) {
  "use strict";
  function hash(text) {
    let value = 2166136261;
    for (const ch of String(text)) {
      value ^= ch.charCodeAt(0);
      value = Math.imul(value, 16777619);
    }
    return value >>> 0;
  }
  function seedFor(day, group, game, round, entry) {
    return hash([day, group, game, round, entry].join("|"));
  }
  function entrySeed() {
    if (root.crypto && root.crypto.getRandomValues) {
      const bits = new Uint32Array(1);
      root.crypto.getRandomValues(bits);
      return bits[0];
    }
    return (Date.now() ^ Math.floor(Math.random() * 4294967296)) >>> 0;
  }
  function varyQuestion(q, seed, random) {
    const rng = random((seed ^ 0xa4b1c987) >>> 0),
      pick = (xs) => xs[Math.floor(rng() * xs.length)];
    const palette = ["#e97e68", "#5799be", "#e0b849", "#68a585", "#9b86bd"];
    q.presentation = {
      surface: pick(["#fffaf0", "#f2f7ed", "#eff6f7", "#faf1ed", "#f5f0f8"]),
      radius: 20 + Math.floor(rng() * 10),
      itemColor: pick(["#e87961", "#83ad68", "#e6b85c"]),
      itemGap: 4 + Math.floor(rng() * 5),
      rotation: Array.from({ length: 24 }, () => Math.floor(rng() * 17) - 8),
    };
    if (q.game === "colors") {
      const shades = {
        coral: ["#e46b5b", "#df6357", "#e67a69"],
        blue: ["#548fc5", "#5b9cce", "#478bbb"],
        yellow: ["#edc14c", "#e6b73e", "#efc656"],
        green: ["#60a277", "#6caa80", "#56976c"],
      };
      for (const option of q.options) option.color = pick(shades[option.id]);
      q.scene.color = q.options[q.answer].color;
    }
    const shared = pick(palette);
    for (const option of q.options || []) {
      if (option.type === "shape")
        option.color = q.game === "odd" ? shared : pick(palette);
      if (option.type === "sized") {
        const base = option.size;
        option.size =
          base < 0.5
            ? 0.4 + rng() * 0.08
            : base < 0.8
              ? 0.63 + rng() * 0.08
              : 0.91 + rng() * 0.08;
        option.color = shared;
      }
    }
    if (q.scene && q.scene.type === "shape" && q.game !== "shadow")
      q.scene.color = pick(palette);
    return q;
  }
  const api = { hash, seedFor, entrySeed, varyQuestion };
  if (typeof module !== "undefined") module.exports = api;
  else root.Variation = api;
})(typeof window !== "undefined" ? window : globalThis);
