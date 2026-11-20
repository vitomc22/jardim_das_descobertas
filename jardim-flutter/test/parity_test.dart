// Paridade com o JS original: test/js_reference.json foi gerado rodando o
// curriculum.js do APK antigo (mesmo arquivo usado pelas versões Rust e Qt).
import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:jardim/curriculum.dart';

void main() {
  test('RNG e hash iguais ao JS', () {
    final r = Rng(42);
    expect(r.next(), 0.6011037519201636);
    expect(r.next(), 0.44829055899754167);
    expect(r.next(), 0.8524657934904099);
    expect(hash('a|b'), 692878806);
    expect(seedFor('2026-11-19', 'little', 'colors', 1, 7), 987461503);
  });

  test('96 perguntas iguais ao JS original', () {
    final ref = jsonDecode(File('test/js_reference.json').readAsStringSync()) as Map<String, dynamic>;
    var ok = 0;
    for (final g in games) {
      for (final grp in ['little', 'middle', 'big']) {
        final r = ref['${g.id}:$grp'] as Map<String, dynamic>;
        final q = generate(g.id, grp, 12345);
        expect(q.prompt, r['prompt'], reason: '${g.id}:$grp prompt');
        expect(q.memory ? q.cards.length : q.options.length, r['n'], reason: '${g.id}:$grp n');
        if (!q.memory) expect(q.answer, r['answer'], reason: '${g.id}:$grp answer');
        expect(q.presentation.surface, r['surface'], reason: '${g.id}:$grp surface');
        ok++;
      }
    }
    expect(ok, 96);
  });
}
