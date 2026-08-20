use serde::{Deserialize, Serialize};

/// A candidate entity the AI might be referring to.
/// The AI layer never sees IDs; the app supplies these and matches on names.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Resolution {
    /// Exactly one match.
    Resolved(String),
    /// Several plausible matches. Ask the user; never pick one.
    Ambiguous(Vec<Candidate>),
    /// Nothing matched.
    NotFound,
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// Resolve a name to an entity id.
///
/// Matching is deliberately staged from strict to loose, and stops at the
/// first stage that yields exactly one result. This means an exact match
/// always wins over a fuzzy one, so "DSA" resolving to a goal literally
/// called "DSA" is never derailed by "DSA Java" also existing.
pub fn resolve(reference: &str, candidates: &[Candidate]) -> Resolution {
    let needle = reference.trim().to_lowercase();
    if needle.is_empty() {
        return Resolution::NotFound;
    }

    // Stage 1: exact, case-insensitive.
    let exact: Vec<&Candidate> = candidates
        .iter()
        .filter(|c| c.name.to_lowercase() == needle)
        .collect();
    if exact.len() == 1 {
        return Resolution::Resolved(exact[0].id.clone());
    }
    if exact.len() > 1 {
        return Resolution::Ambiguous(exact.into_iter().cloned().collect());
    }

    // Stage 2: candidate starts with the reference.
    let prefix: Vec<&Candidate> = candidates
        .iter()
        .filter(|c| c.name.to_lowercase().starts_with(&needle))
        .collect();
    if prefix.len() == 1 {
        return Resolution::Resolved(prefix[0].id.clone());
    }
    if prefix.len() > 1 {
        return Resolution::Ambiguous(prefix.into_iter().cloned().collect());
    }

    // Stage 3: substring, either direction.
    let contains: Vec<&Candidate> = candidates
        .iter()
        .filter(|c| {
            let name = c.name.to_lowercase();
            name.contains(&needle) || needle.contains(&name)
        })
        .collect();
    if contains.len() == 1 {
        return Resolution::Resolved(contains[0].id.clone());
    }
    if contains.len() > 1 {
        return Resolution::Ambiguous(contains.into_iter().cloned().collect());
    }

    // Stage 4: all significant words present, in any order.
    let words: Vec<&str> = needle.split_whitespace().filter(|w| w.len() > 2).collect();
    if !words.is_empty() {
        let word_match: Vec<&Candidate> = candidates
            .iter()
            .filter(|c| {
                let name = c.name.to_lowercase();
                words.iter().all(|w| name.contains(w))
            })
            .collect();
        if word_match.len() == 1 {
            return Resolution::Resolved(word_match[0].id.clone());
        }
        if word_match.len() > 1 {
            return Resolution::Ambiguous(word_match.into_iter().cloned().collect());
        }
    }

    Resolution::NotFound
}

/// Build the clarification question shown when a reference is ambiguous.
pub fn clarification_question(reference: &str, candidates: &[Candidate]) -> String {
    format!(
        "\"{}\" could mean {} things. Which did you mean?",
        reference,
        candidates.len()
    )
}