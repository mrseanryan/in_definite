use crate::string_helper;

pub fn is_acronym(word: &str) -> bool {
    word.chars().all(|ch| ch.is_uppercase())
}

// ref: https://github.com/tandrewnichols/indefinite/blob/master/lib/rules/acronyms.js
pub fn is_an_for_acronym(word: &str) -> bool {
    let is_irregular = is_irregular_acronym(word);
    let initial_vowel = starts_with_vowel(word);
    /*
     * If it starts with U: "a"
     * If it starts with any other vowel: "an"
     * If it starts with F, H, L, M, N, R, S, or X: "an"
     * If it starts with any other consonant: "a"
     */
    if both_or_neither(initial_vowel, is_irregular) {
        return false;
    }
    true
}

fn both_or_neither(a: bool, b: bool) -> bool {
    a && b || !a && !b
}

fn is_irregular_acronym(word: &str) -> bool {
    "UFHLMNRSX".contains(string_helper::get_first_letter(word))
}

fn starts_with_vowel(word: &str) -> bool {
    "aeiouAEIOU".contains(string_helper::get_first_letter(word))
}
