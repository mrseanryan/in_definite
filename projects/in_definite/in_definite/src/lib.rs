//! # in_definite
//!
//! Get the indefinite article ('a' or 'an') to match the given word. For example: an umbrella, a user.

use regex::Regex;

pub struct Options {
    /// If true, then a 4 digit number like '1800' is treated like 'eighteen hundred', so will use 'an'.
    /// 
    /// Normally, such a number is treated like 'one thousand eight hundred', so would use 'a'.
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

// Traits required for the unit test assertions:
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Is
{
    An,
    A,
    None
}

/// Get 'a' or 'an' to match the given word.
///
/// # Examples
///
/// ```
/// use in_definite;
/// 
/// let result = in_definite::get_a_or_an("alien");
///
/// assert_eq!("an", result);
/// ```
///
/// ```
/// // Irregular word
/// let result = in_definite::get_a_or_an("unicorn");
///
/// assert_eq!("a", result);
/// ```
///
/// ```
/// // Title Case
/// let result = in_definite::get_a_or_an("Ugly");
///
/// assert_eq!("An", result);
/// ```
pub fn get_a_or_an(word: &str) -> &str {
    get_a_or_an_options(word, &Options::default())
}

/// Get 'a' or 'an' to match the given word, with options.
///
/// # Examples
///
/// ```
/// use in_definite;
/// 
/// // 'eighteen hundred'
/// let result = in_definite::get_a_or_an_options("1800", &in_definite::Options::with_colloquial());
///
/// assert_eq!("an", result);
/// ```
/// 
/// ```
/// // 'one thousand eight hundred'
/// let result = in_definite::get_a_or_an_options("1800", &in_definite::Options::default());
///
/// assert_eq!("a", result);
/// ```
/// 
/// ```
/// // Title Case
/// let result = in_definite::get_a_or_an_options("Ugly", &in_definite::Options::default());
///
/// assert_eq!("An", result);
/// ```
pub fn get_a_or_an_options(word: &str, options: &Options) -> &'static str {
    if word.trim().len() == 0 {
        return "";
    }

    let is_an = is_an_options(word, options);

    a_or_an_capitalized_to_match(is_an, get_first_word(word)) 
}

fn a_or_an_capitalized_to_match(is_an: Is, first_word: &str) -> &'static str {  
    let is_title_case = is_title_case(first_word);

    match is_an {
        Is::An => {
        if is_title_case {
            return "An";
        }

        return "an";
        },
        Is::A => {
            if is_title_case {
                return "A";
            }

            "a"
        },
        _ => ""
    }
}

// TODO xxx simplify via regex
fn is_title_case(first_word: &str) -> bool {
    starts_with_capital(first_word) && !remainder_has_capitals(first_word)
}

fn starts_with_capital(first_word: &str) -> bool {
    is_capital_char(get_first_letter(first_word))
}

fn remainder_has_capitals(first_word: &str) -> bool {
    let mut chars_iter = first_word.chars();
    chars_iter.next();
    chars_iter.any(|c| is_capital_char(c))
}

fn is_capital_char(c: char) -> bool {
    'A' <= c && c <= 'Z'
}

/// Returns true if the given word should be used with 'an' (not 'a').
///
/// # Examples
///
/// ```
/// use in_definite;
/// 
/// let result = in_definite::is_an("alien");
///
/// assert_eq!(in_definite::Is::An, result);
/// ```
///
/// ```
/// let result = in_definite::is_an("unicorn");
///
/// assert_eq!(in_definite::Is::A, result);
/// ```
/// 
/// ```
/// let result = in_definite::is_an("");
///
/// assert_eq!(in_definite::Is::None, result);
/// ```
pub fn is_an(word: &str) -> Is {
    is_an_options(word, &Options::default())
}

/// Get 'a' or 'an' to match the given word, with options.
///
/// # Examples
///
/// ```
/// use in_definite;
/// 
/// let result = in_definite::is_an_options("1800", &in_definite::Options::with_colloquial()); // 'eighteen hundred'
///
/// assert_eq!(in_definite::Is::An, result);
/// ```
///
/// ```
/// let result = in_definite::is_an_options("1800", &in_definite::Options::default()); // 'one thousand eight hundred'
///
/// assert_eq!(in_definite::Is::A, result);
/// ```
///
/// ```
/// let result = in_definite::is_an_options(" ", &in_definite::Options::default());
///
/// assert_eq!(in_definite::Is::None, result);
/// ```
pub fn is_an_options(word: &str, options: &Options) -> Is {
    if word.trim().len() == 0 {
        return Is::None;
    }

    if is_an_options_bool(word, options)
    {
        return Is::An;
    }

    Is::A
}

fn is_an_options_bool(word: &str, options: &Options) -> bool {
    let word = get_first_word(word);

    let word_lower = word.to_lowercase();

    if is_number(word) {
        return is_an_for_number(word, options)
    }

    let is_an_result = is_naively_an(&word_lower);

    if is_acronym(word) {
        return is_an_for_acronym(word);
    }

    if is_exception(&word_lower) || is_exception_after_strip(&word_lower) {
        return !is_an_result;
    }

    is_an_result
}

fn is_exception_after_strip(word_lower: &str) -> bool {
    // into_iter(): 
    // book: Rust 2018 says: If we want to create an iterator that takes ownership of v1 and returns owned values, we can call into_iter instead of iter. 
    ["s", "es", "ed", "ly"].into_iter().map(|ending| strip_end(&word_lower, ending))
        .any(|stripped| is_exception(stripped))
}

fn strip_end<'s>(word: &'s str, ending: &str) -> &'s str {
    if word.ends_with(ending) {
        return &word[..(word.len() - ending.len())]
    }

    word
}

fn get_first_word(word: &str) -> &str {
    let word = word.trim();

    let words: Vec<&str> = word.split(|c: char| " ,.-;:'".contains(c)).collect();

    words[0]
}

fn get_first_letter(word: &str) -> char {
    word.chars().next().unwrap()
}

fn is_naively_an(word: &str) -> bool {
    "aeiou".contains(get_first_letter(word))
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
        "euro",
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
        "urologist",
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
        // (handled generically)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_or_an_capitalized_to_match_test() {
        // Title case - should match
        assert_eq!("An", a_or_an_capitalized_to_match(Is::An, "Ugly"));
        assert_eq!("A", a_or_an_capitalized_to_match(Is::A, "Leopard"));
        // lower case - do nothing
        assert_eq!("an", a_or_an_capitalized_to_match(Is::An, "ugly"));
        assert_eq!("a", a_or_an_capitalized_to_match(Is::A, "leopard"));
        // MiXed case - do nothing
        assert_eq!("an", a_or_an_capitalized_to_match(Is::An, "UgLy"));
        assert_eq!("a", a_or_an_capitalized_to_match(Is::A, "lEoparD"));
        // UPPER case - do nothing (acronym)
        assert_eq!("an", a_or_an_capitalized_to_match(Is::An, "FIFA"));
        assert_eq!("a", a_or_an_capitalized_to_match(Is::A, "UN"));
    }

    #[test]
    fn get_first_word_test() {
        assert_eq!("one", get_first_word("one two"));
        assert_eq!("one", get_first_word("one two three"));
        assert_eq!("one", get_first_word("one-two three"));
        assert_eq!("heir", get_first_word("heir's"));
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

        assert_eq!(Is::An, is_an("antelope"));
        assert_eq!(Is::An, is_an("apple"));
        assert_eq!(Is::A, is_an("pear"));
    }

    #[test]
    fn zero_length() {
        assert_eq!("", get_a_or_an(""));
        assert_eq!(Is::None, is_an(""));
    }

    #[test]
    fn all_whitespace() {
        assert_eq!("", get_a_or_an("   "));
        assert_eq!(Is::None, is_an("   "));
    }

    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, get_a_or_an(input));
                assert_eq!(expected.to_lowercase() == "an", is_an(input) == Is::An);
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
                assert_eq!(expected.to_lowercase() == "an", is_an_options(input, options) == Is::An);
            }
        )*
        }
    }

    tests! {
        // acronymns
        test_ac0: ("CEO", "a"),
        test_ac1: ("EU", "an"),
        test_ac2a: ("FFA", "an"),
        test_ac2b: ("FIFA", "an"),
        test_ac2c: ("IOU", "an"),
        test_ac3: ("MIA", "an"),
        test_ac4: ("MNM", "an"),
        test_ac5: ("UFO", "a"),
        test_ac6: ("UN", "a"),
        test_ac7: ("US", "a"),
        test_ac8: ("USA", "a"),
        // words
        test_a1: ("alien", "an"),
        test_a2a: ("american", "an"),
        test_a2b: ("antelope", "an"),
        test_a2c: ("apple", "an"),
        test_a2d: ("banana", "a"),
        test_e1: ("economic", "an"),
        test_e2: ("economy", "an"),
        // nouns eu-
        test_eu1: ("euro", "a"),
        test_eu2: ("european", "a"),
        test_eu3: ("European", "A"),
        test_eu4: ("eucalyptus", "a"),
        test_eu5: ("eulogy", "a"),
        // nouns that begin with ur-.
        test_ur1: ("uranium", "a"),
        test_ur2: ("urinal", "a"),
        test_ur3: ("urologist", "a"),
        // uni - Most nouns or adjectives that begin with uni– also require the indefinite article a.
        test_uni1: ("unicorn", "a"),
        test_uni2: ("uniform", "a"),
        test_uni3: ("unit", "a"),
        test_uni4: ("universal", "a"),
        test_uni5: ("university", "a"),
        // uni - adjectives that begin with uni– which follow the general rule.
        test_uni_adj1: ("unidentified", "an"),
        test_uni_adj2: ("unimportant", "an"),
        test_uni_adj3: ("unintended", "an"),
        test_uni_adj4: ("unintelligent", "an"),
        // more words
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
        test_n11: ("11", "an"),
        test_n12: ("12", "a"),
        test_n13: ("13", "a"),
        test_n14: ("14", "a"),
        test_n15: ("15", "a"),
        test_n16: ("16", "a"),
        test_n17: ("17", "a"),
        test_n18: ("18", "an"),
        test_n19: ("19", "a"),
        test_n20: ("20", "a"),
        // numbers - years
        test_ny1000: ("1000", "a"),
        test_ny1800: ("1800", "a"),
        test_ny1892: ("1892", "a"),
        // starting with 11
        test_n11_01: ("11", "an"),
        test_n11_02: ("110", "a"),
        test_n11_03: ("1100", "a"), // 'one thousand one hundred'
        test_n11_04: ("11000", "an"),
        test_n11_05: ("110000", "a"),
        test_n11_06: ("1100000", "a"),
        test_n11_07: ("11000000", "an"),
        test_n11_08: ("110000000", "a"),
        test_n11_09: ("1100000000", "a"),
        test_n11_10: ("11000000000", "an"),
        test_n11_11: ("110000000000", "a"),
        test_n11_12: ("1100000000000", "a"),
        test_n11_13: ("11000000000000", "an"),
        test_n11_14: ("110000000000000", "a"),
        test_n11_15: ("1100000000000000", "a"),
        test_n11_16: ("11000000000000000", "an"),
        // starting with 18
        test_ns18_01: ("18", "an"),
        test_ns18_02: ("180", "a"),
        test_ns18_03: ("18000", "an"),
        test_ns18_04: ("180000", "a"),
        test_ns18_05: ("1800000", "a"),
        test_ns18_06: ("18000000", "an"),
        test_ns18_07: ("180000000", "a"),
        test_ns18_08: ("1800000000", "a"),
        test_ns18_09: ("18000000000", "an"),
        test_ns18_10: ("180000000000", "a"),
        test_ns18_11: ("1800000000000", "a"),
        test_ns18_12: ("18000000000000", "an"),
        test_ns18_13: ("180000000000000", "a"),
        test_ns18_14: ("1800000000000000", "a"),
        test_ns18_15: ("18000000000000000", "an"),
        // starting with 8
        test_n8_01: ("8", "an"),
        test_n8_02: ("80", "an"),
        test_n8_03: ("800", "an"),
        test_n8_04: ("8000", "an"),
        test_n8_05: ("80000", "an"),
        test_n8_06: ("800000", "an"),
        test_n8_07: ("8000000", "an"),
        test_n8_08: ("80000000", "an"),
        test_n8_09: ("800000000", "an"),
        // Mixed case
        test_mc1: ("Alien", "An"), // Title Case
        test_mc2: ("anteLoPe", "an"), // mixed case
        test_mc3: ("haiR", "a"), // mixed case
        test_mc4: ("HEIR", "an"), // acronym 'h -> an'
        test_mc5: ("Heir", "An"), // Title Case, irregular
        test_mc6: ("Ugly", "An"), // Title Case
        ////////////////
        // other: words with spaces or hyphens, plurals etc.
        // 2 words
        test_other1: ("ouija board", "a"),
        // hyphenation
        test_other2: ("apple-board", "an"),
        test_other3: ("honor-bound", "an"),
        test_other4: ("horror-bound", "a"),
        // suffix
        test_other_s1: ("heavenly", "a"),
        test_other_s2: ("honored", "an"),
        test_other_s3: ("hourly", "an"),
        test_other_s4: ("heirly", "an"),
        // plural
        test_other6: ("heiresses", "an"),
        test_other6b: ("heirs", "an"),
        test_other7: ("honors", "an"),
        // possessive
        test_other8: ("heir's", "an"),
        test_other9: ("horror's", "a"),
        // Adverbs: u like y
        test_other_adv1: ("ubiquitously", "a"),
        test_other_adv2: ("ukelele", "a"),
        test_other_adv3: ("unanimously", "a"),
        test_other_adv4: ("unicamerally", "a"),
        test_other_adv5: ("uniquely", "a"),
        test_other_adv6: ("universally", "a"),
        test_other_adv7: ("urologically", "a"),
        test_other_adv8: ("usefully", "a"),
        test_other_adv9: ("uselessly", "a"),
        test_other_adv10: ("usuriously", "a"),
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
        test_colloquial_ny1000: ("1000", "a"), // 'ten hundred'
        test_colloquial_ny1100: ("1100", "an"), // 'eleven hundred'
        test_colloquial_ny1800: ("1800", "an"),
        test_colloquial_ny1892: ("1892", "an"),
    }
}
