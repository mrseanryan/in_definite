//! # in_definite
//!
//! Get the indefinite article (a or an) for the given word.

use regex::Regex;

pub struct Options {
    pub are_numbers_colloquial: bool,
}

impl Options {
    pub fn default() -> Options {
        Options {are_numbers_colloquial: false}
    }

    pub fn with_colloquial() -> Options {
        Options {are_numbers_colloquial: true}
    }
}

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
    get_a_or_an_options(word, &Options::default())
}

/// Get 'a' or 'an' to match the given word, with options.
///
/// # Examples
///
/// ```
/// let result = in_definite::get_a_or_an_options("1800", &in_definite::Options::with_colloquial()); // 'eighteen hundred'
///
/// assert_eq!("an", result);
/// ```
/// 
/// ```
/// let result = in_definite::get_a_or_an_options("1800", &in_definite::Options::default()); // 'one thousand eight hundred'
///
/// assert_eq!("a", result);
/// ```
pub fn get_a_or_an_options<'s>(word: &'s str, options: &Options) -> &'s str {
    if word.len() == 0 {
        return "";
    }

    if is_an_options(word, options) {
        return "an";
    }
    
    "a"    
}

/// Returns true if the given word should be used with 'an' (not 'a').
///
/// # Examples
///
/// ```
/// let result = in_definite::is_an("alien");
///
/// assert_eq!(true, result);
/// ```
///
/// ```
/// let result = in_definite::is_an("unicorn");
///
/// assert_eq!(false, result);
/// ```
pub fn is_an(word: &str) -> bool {
    is_an_options(word, &Options::default())
}

/// Get 'a' or 'an' to match the given word, with options.
///
/// # Examples
///
/// ```
/// let result = in_definite::is_an_options("1800", &in_definite::Options::with_colloquial()); // 'eighteen hundred'
///
/// assert_eq!(true, result);
/// ```
///
/// ```
/// let result = in_definite::is_an_options("1800", &in_definite::Options::default()); // 'one thousand eight hundred'
///
/// assert_eq!(false, result);
/// ```
pub fn is_an_options(word: &str, options: &Options) -> bool {
    if word.len() == 0 {
        return false;
    }

    let word = get_first_word(word);

    if is_number(word) {
        return is_an_for_number(word, options)
    }

    let is_an_result = is_naively_an(word);

    if is_acronym(word) {
        return is_an_for_acronym(word);
    }

    // TODO refactor to avoid duplication
    if is_exception(word) || is_exception(strip_end(word, "s")) || is_exception(strip_end(word, "es")) || is_exception(strip_end(word, "ed")) {
        return !is_an_result;
    }

    return is_an_result
}

fn strip_end<'s>(word: &'s str, ending: &str) -> &'s str {
    if word.ends_with(ending) {
        return &word[..(word.len() - ending.len())]
    }

    word
}

fn get_first_word(word: &str) -> &str {
    let word = word.trim();

    let words: Vec<&str> = word.split(|c: char| " ,.-;:".contains(c)).collect();

    words[0]
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

    exceptions.contains(&word)
}

fn is_acronym(word: &str) -> bool {
    is_match(word, r"^[A-Z]+$")
}

// ref: https://github.com/tandrewnichols/indefinite/blob/master/lib/rules/acronyms.js
fn is_an_for_acronym(word: &str) -> bool {
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
    is_match(word, r"^[UFHLMNRSX]")
}

fn starts_with_vowel(word: &str) -> bool {
    is_match(word, r"^[aeiouAEIOU]")
}

fn is_match(word: &str, regex: &str) -> bool {
    let re = Regex::new(regex).unwrap();
    re.is_match(word)
}

// numbers
// ref: https://github.com/tandrewnichols/indefinite/blob/master/lib/rules/numbers.js
fn is_number(word: &str) -> bool {
    is_match(word, r"^([0-9,]+)")
}

fn is_an_for_number(word: &str, options: &Options) -> bool {
    let mut is_an = false;

    if is_match(word, r"^(11|8|18)") {
        let starts_with_11_or_18 = is_match(word, r"^(11|18)");

        // If the number starts with 11 or 18 and is of length 4,
        // the pronunciation is ambiguous so check opts.numbers to see
        // how to render it. Otherwise, if it starts with 11 or 18
        // and has 2, 5, 8, 11, etc. digits, use 'an.' Finally, if it
        // starts with an 8, use 'an.' For everything else, use 'a.'
        if starts_with_11_or_18 && word.len() == 4 {
            is_an = options.are_numbers_colloquial;
        } else if starts_with_11_or_18 && (word.len() - 2) % 3 == 0 {
            is_an = true;
        } else {
            is_an = word.starts_with('8');
        }
    }

    is_an
}

// TODO other
// ref: https://github.com/tandrewnichols/indefinite/blob/master/lib/rules/other.js

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_first_word_test() {
        assert_eq!("one", get_first_word("one two"));
        assert_eq!("one", get_first_word("one two three"));
        assert_eq!("one", get_first_word("one-two three"));
    }

    #[test]
    fn strip_end_test() {
        assert_eq!("one", strip_end("ones", "s"));
        assert_eq!("heir", strip_end("heir's", "'s"));
        assert_eq!("hour", strip_end("houred", "ed"));
        assert_eq!("hour", strip_end("hourly", "ly"));
        assert_eq!("hour", strip_end("hour's", "'s"));
    }

    #[test]
    fn common_words() {
        assert_eq!("an", get_a_or_an("antelope"));
        assert_eq!("an", get_a_or_an("apple"));
        assert_eq!("a", get_a_or_an("pear"));

        assert_eq!(true, is_an("antelope"));
        assert_eq!(true, is_an("apple"));
        assert_eq!(false, is_an("pear"));
    }

    #[test]
    fn zero_length() {
        assert_eq!("", get_a_or_an(""));
        assert_eq!(false, is_an(""));
    }

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, get_a_or_an(input));
                assert_eq!(expected == "an", is_an(input));
            }
        )*
        }
    }

    macro_rules! tests_options_with_colloquial {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let options = &(Options::with_colloquial());

                let (input, expected) = $value;
                assert_eq!(expected, get_a_or_an_options(input, options));
                assert_eq!(expected == "an", is_an_options(input, options));
            }
        )*
        }
    }

    tests! {
        test_ac0: ("CEO", "a"),
        test_ac1: ("EU", "an"),
        test_ac2: ("FIFA", "an"),
        test_ac2b: ("IOU", "an"),
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
        test_o0: ("ordinary", "an"),
        test_o1: ("ouija", "a"),
        test_u0: ("ukelele", "a"),
        test_u1: ("umbrella", "an"),
        test_u2: ("user", "a"),
        // numbers
        test_n0: ("0", "a"),
        test_n1: ("1", "a"),
        test_n2: ("2", "a"),
        test_n3: ("3", "a"),
        test_n4: ("4", "a"),
        test_n5: ("5", "a"),
        test_n6: ("6", "a"),
        test_n7: ("7", "a"),
        test_n8: ("8", "an"),
        test_n9: ("9", "a"),
        test_n10: ("10", "a"),
        // numbers - years
        test_ny1000: ("1000", "a"),
        test_ny1800: ("1800", "a"),
        test_ny1892: ("1892", "a"),
        // other: words with spaces or hyphens, plurals etc.
        // 2 words
        test_other1: ("ouija board", "a"),
        // hyphenation
        test_other2: ("apple-board", "an"),
        test_other3: ("honor-bound", "an"),
        test_other4: ("horror-bound", "a"),
        // suffix
        test_other5: ("heavenly", "a"),
        test_other5b: ("hourly", "an"),
        // plural
        test_other6: ("heiresses", "an"),
        test_other6b: ("heirs", "an"),
        test_other7: ("honors", "an"),
        // possessive
        // TODO xxx fix - test_other8: ("heir's", "an"),
        test_other9: ("horror's", "a"),
    }

    tests_options_with_colloquial! {
        test_colloquial_ac1: ("EU", "an"),
        test_colloquial_ac2: ("FIFA", "an"),
        test_colloquial_ac3: ("MIA", "an"),
        test_colloquial_ac4: ("MNM", "an"),
        test_colloquial_ac5: ("UFO", "a"),
        test_colloquial_ac6: ("UN", "a"),
        test_colloquial_a1: ("alien", "an"),
        test_colloquial_a2: ("antelope", "an"),
        test_colloquial_h1: ("hair", "a"),
        test_colloquial_h2: ("heir", "an"),
        test_colloquial_h3: ("herb", "an"), // USA not UK
        test_colloquial_h4: ("hotel", "a"),
        test_colloquial_u1: ("umbrella", "an"),
        test_colloquial_u2: ("user", "a"),
        // numbers
        test_colloquial_n0: ("0", "a"),
        test_colloquial_n1: ("1", "a"),
        test_colloquial_n2: ("2", "a"),
        test_colloquial_n3: ("3", "a"),
        test_colloquial_n4: ("4", "a"),
        test_colloquial_n5: ("5", "a"),
        test_colloquial_n6: ("6", "a"),
        test_colloquial_n7: ("7", "a"),
        test_colloquial_n8: ("8", "an"),
        test_colloquial_n9: ("9", "a"),
        test_colloquial_n10: ("10", "a"),
        // numbers - years: with colluquial on, they are as read out loud: "an eighteen hundred"
        test_colloquial_ny1000: ("1000", "a"),
        test_colloquial_ny1800: ("1800", "an"),
        test_colloquial_ny1892: ("1892", "an"),
    }
}
