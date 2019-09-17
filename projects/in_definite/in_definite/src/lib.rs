//! # in_definite
//!
//! Get the indefinite article (a or an) for the given word.

use regex::Regex;

/// Get 'a' or 'an' to match the given word.
///
/// # Examples
///
/// ```
/// let result = in_definite::get_a_or_an("alien");
///
/// assert_eq!("an", result);
/// ```
///
/// ```
/// let result = in_definite::get_a_or_an("unicorn");
///
/// assert_eq!("a", result);
/// ```
pub fn get_a_or_an(word: &str) -> &str {
    let mut is_an = is_naively_an(word);

    if is_exception(word) {
        is_an = !is_an;
    }

    if is_acronym(word) {
        return a_or_an_for_acronym(word);
    }

    if is_an {
        return "an";
    } else {
        return "a";
    }
}

fn is_naively_an(word: &str) -> bool {
    let first_letter = word.to_lowercase().chars().next().unwrap();

    if "aeiou".contains(first_letter) {
        return true;
    }

    false
}

fn is_exception(word: &str) -> bool {
    // ref: https://github.com/tandrewnichols/indefinite/blob/master/lib/irregular-words.js

    let exceptions = [
        // Nouns: eu like y
        "eunuch",
        "eucalyptus",
        "eugenics",
        "eulogy",
        "euphemism",
        "euphony",
        "euphoria",
        "eureka",
        // Adjectives: eu like y
        "european",
        "euphemistic",
        "euphonic",
        "euphoric",
        // Adverbs: eu like y
        "euphemistically",
        "euphonically",
        "euphorically",
        // Nouns: silent h
        "heir",
        "heiress",
        "herb",
        "homage",
        "honesty",
        "honor",
        "honour",
        "hour",
        // Adjectives: silent h
        "honest",
        "honorous",
        // Adverbs: silent h
        "honestly",
        "hourly",
        // Nouns: o like w
        "one",
        "ouija",
        // Adjectives: o like w
        "once",
        // Adverbs: o like w

        // Nouns: u like y
        "ubiquity",
        "udometer",
        "ufo",
        "uke",
        "ukelele",
        "ululate",
        "unicorn",
        "unicycle",
        "uniform",
        "unify",
        "union",
        "unison",
        "unit",
        "unity",
        "universe",
        "university",
        "upas",
        "ural",
        "uranium",
        "urea",
        "ureter",
        "urethra",
        "urine",
        "urology",
        "urus",
        "usage",
        "use",
        "user",
        "usual",
        "usurp",
        "usury",
        "utensil",
        "uterus",
        "utility",
        "utopia",
        "utricle",
        "uvarovite",
        "uvea",
        "uvula",
        // Adjectives: u like y
        "ubiquitous",
        "ugandan",
        "ukrainian",
        "unanimous",
        "unicameral",
        "unified",
        "unique",
        "unisex",
        "universal",
        "urinal",
        "urological",
        "useful",
        "useless",
        "usurious",
        "usurped",
        "utilitarian",
        "utopic",
        // Adverbs: u like y
        "ubiquitously",
        "unanimously",
        "unicamerally",
        "uniquely",
        "universally",
        "urologically",
        "usefully",
        "uselessly",
        "usuriously",
        // Nouns: y like i
        "yttria",
        "yggdrasil",
        "ylem",
        "yperite",
        "ytterbia",
        "ytterbium",
        "yttrium",
        // Adjectives: y like i
        "ytterbous",
        "ytterbic",
        "yttric",
    ];

    return exceptions.contains(&word);
}

fn is_acronym(word: &str) -> bool {
    let re = Regex::new(r"^[A-Z]+$").unwrap();
    return re.is_match(word);
}

// ref: https://github.com/tandrewnichols/indefinite/blob/master/lib/rules/acronyms.js
fn a_or_an_for_acronym(word: &str) -> &str {
    let is_irregular = is_irregular_acronym(word);
    let initial_vowel = starts_with_vowel(word);
    /*
     * If it starts with U: "a"
     * If it starts with any other vowel: "an"
     * If it starts with F, H, L, M, N, R, S, or X: "an"
     * If it starts with any other consonant: "a"
     */
    let mut article = "an";
    if both_or_neither(initial_vowel, is_irregular) {
        article = "a";
    }
    return article;
}

fn both_or_neither(a: bool, b: bool) -> bool {
    return a && b || !a && !b;
}

fn is_irregular_acronym(word: &str) -> bool {
    let re = Regex::new(r"^[UFHLMNRSX]").unwrap();
    return re.is_match(word);
}

fn starts_with_vowel(word: &str) -> bool {
    let re = Regex::new(r"^[aeiouAEIOU]").unwrap();
    return re.is_match(word);
}

// TODO numbers
// ref: https://github.com/tandrewnichols/indefinite/blob/master/lib/rules/numbers.js

// TODO other
// ref: https://github.com/tandrewnichols/indefinite/blob/master/lib/rules/other.js

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_words() {
        assert_eq!("an", get_a_or_an("antelope"));
    }

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, get_a_or_an(input));
            }
        )*
        }
    }

    tests! {
        test_ac1: ("EU", "an"),
        test_ac2: ("FIFA", "an"),
        test_ac3: ("MIA", "an"),
        test_ac4: ("MNM", "an"),
        test_ac5: ("UFO", "a"),
        test_ac6: ("UN", "a"),
        test_a1: ("alien", "an"),
        test_a2: ("antelope", "an"),
        test_h1: ("hair", "a"),
        test_h2: ("heir", "an"),
        test_h3: ("herb", "an"), // USA not UK
        test_h4: ("hotel", "a"),
        test_u1: ("umbrella", "an"),
        test_u2: ("user", "a"),
    }
}
