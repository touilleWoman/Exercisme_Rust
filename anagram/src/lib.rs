use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut ret: HashSet<&'a str> = HashSet::new();
    let word: String = word.to_lowercase();
    let word_set: HashSet<_> = word.chars().collect();
    for candidate in possible_anagrams {
        let low_candidate = candidate.to_lowercase();
        if word == low_candidate {
            continue;
        }
        let mut word2 = word.clone().into_bytes();
        word2.sort_unstable();
        let mut candidate2 = low_candidate.clone().into_bytes();
        candidate2.sort_unstable();
        if word2 != candidate2 {
            continue;
        }
        let candidate_set: HashSet<_> = low_candidate.chars().collect();
        if word_set.is_subset(&candidate_set) {
            ret.insert(candidate);
        }
    }
    ret
}
