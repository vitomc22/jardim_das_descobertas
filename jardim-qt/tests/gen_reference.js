const A = process.argv[2];
const C = require(A + "/curriculum.js"), V = require(A + "/variation.js");
const r = C.random(42); console.log(JSON.stringify([r(), r(), r()]));
console.log(V.hash("a|b"), V.seedFor("2026-11-19","little","colors",1,7));
const out = {};
for (const g of C.GAMES) for (const grp of ["little","middle","big"]) {
  const q = C.generate(g.id, grp, 12345);
  out[g.id+":"+grp] = {prompt:q.prompt, answer:q.answer, n:(q.options||q.cards).length, surface:q.presentation.surface};
}
console.log(JSON.stringify(out));
