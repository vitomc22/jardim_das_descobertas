//! Paridade com o JS original: tests/js_reference.json foi gerado rodando o
//! curriculum.js do APK antigo (tests/gen_reference.js) com a semente 12345.
use jardim::curriculum::{generate, GAMES};

#[test]
fn same_questions_as_javascript() {
    let raw = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/js_reference.json")).unwrap();
    let reference: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let mut checked = 0;
    for g in GAMES.iter() {
        for grp in ["little", "middle", "big"] {
            let r = &reference[format!("{}:{grp}", g.id)];
            let q = generate(g.id, grp, 12345);
            let n = if q.memory { q.cards.len() } else { q.options.len() };
            assert_eq!(q.prompt, r["prompt"].as_str().unwrap(), "{}:{grp} prompt", g.id);
            assert_eq!(n as u64, r["n"].as_u64().unwrap(), "{}:{grp} n", g.id);
            if !q.memory {
                assert_eq!(q.answer as u64, r["answer"].as_u64().unwrap(), "{}:{grp} answer", g.id);
            }
            assert_eq!(q.presentation.surface, r["surface"].as_str().unwrap(), "{}:{grp} surface", g.id);
            checked += 1;
        }
    }
    assert_eq!(checked, 96);
}
