use stark_ai::resolver::{resolve, Candidate, Resolution};

fn c(id: &str, name: &str) -> Candidate {
    Candidate { id: id.into(), name: name.into() }
}

fn goals() -> Vec<Candidate> {
    vec![
        c("g1", "DSA Java"),
        c("g2", "DSA Python"),
        c("g3", "Research Project"),
        c("g4", "University Setup"),
    ]
}

#[test]
fn exact_match_resolves() {
    assert_eq!(
        resolve("DSA Java", &goals()),
        Resolution::Resolved("g1".into())
    );
}

#[test]
fn matching_is_case_insensitive() {
    assert_eq!(
        resolve("dsa java", &goals()),
        Resolution::Resolved("g1".into())
    );
}

#[test]
fn ambiguous_prefix_asks_rather_than_guesses() {
    // "DSA" matches two goals. This must NOT silently pick the first.
    match resolve("DSA", &goals()) {
        Resolution::Ambiguous(cands) => assert_eq!(cands.len(), 2),
        other => panic!("expected Ambiguous, got {other:?}"),
    }
}

#[test]
fn unique_prefix_resolves() {
    assert_eq!(
        resolve("Research", &goals()),
        Resolution::Resolved("g3".into())
    );
}

#[test]
fn substring_resolves_when_unique() {
    assert_eq!(
        resolve("university", &goals()),
        Resolution::Resolved("g4".into())
    );
}

#[test]
fn word_match_ignores_order() {
    assert_eq!(
        resolve("java dsa", &goals()),
        Resolution::Resolved("g1".into())
    );
}

#[test]
fn no_match_returns_not_found() {
    assert_eq!(resolve("Physics", &goals()), Resolution::NotFound);
}

#[test]
fn empty_reference_returns_not_found() {
    assert_eq!(resolve("", &goals()), Resolution::NotFound);
    assert_eq!(resolve("   ", &goals()), Resolution::NotFound);
}

#[test]
fn empty_candidate_list_returns_not_found() {
    assert_eq!(resolve("anything", &[]), Resolution::NotFound);
}

#[test]
fn exact_match_beats_ambiguous_prefix() {
    // A goal literally called "DSA" should win, even though
    // "DSA Java" and "DSA Python" also start with it.
    let mut list = goals();
    list.push(c("g5", "DSA"));
    assert_eq!(resolve("DSA", &list), Resolution::Resolved("g5".into()));
}

#[test]
fn duplicate_names_are_ambiguous() {
    let list = vec![c("g1", "Study"), c("g2", "Study")];
    match resolve("Study", &list) {
        Resolution::Ambiguous(cands) => assert_eq!(cands.len(), 2),
        other => panic!("expected Ambiguous, got {other:?}"),
    }
}