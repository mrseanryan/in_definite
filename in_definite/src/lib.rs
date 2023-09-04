//! # in_definite
//!
//! Get the indefinite article ('a' or 'an') to match the given word. For example: an umbrella, a user.

mod core_is_an;
mod options;
mod rules;
mod utils;

use utils::string_helper;

pub use core_is_an::Is;
pub use options::Options;

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

    core_is_an::a_or_an_capitalized_to_match(is_an, string_helper::get_first_word(word))
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

    if core_is_an::is_an_options_bool(word, options) {
        return Is::An;
    }

    Is::A
}

#[cfg(test)]
mod tests {
    use super::*;

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
        // acronyms
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
        test_r1: ("red", "a"),
        test_r2: ("red rum", "a"),
        test_r3: ("rum", "a"),
        test_u0: ("ukelele", "a"),
        test_u1: ("umbrella", "an"),
        test_u2: ("user", "a"),
        test_u2a: ("usurper", "a"),
        test_u2b: ("usurped", "a"),
        test_u3: ("Utah", "A"),
        test_u4: ("utahn", "a"),
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
        // numbers - thousands with separator
        test_n1_000: ("1,000", "a"),
        test_n2_000: ("2,000", "a"),
        test_n3_000: ("3,000", "a"),
        test_n4_000: ("4,000", "a"),
        test_n5_000: ("5,000", "a"),
        test_n6_000: ("6,000", "a"),
        test_n7_000: ("7,000", "a"),
        test_n8_000: ("8,000", "an"),
        test_n9_000: ("9,000", "a"),
        test_n10_000: ("10,000", "a"),
        test_n11_000: ("11,000", "an"),
        test_n12_000: ("12,000", "a"),
        test_n13_000: ("13,000", "a"),
        test_n14_000: ("14,000", "a"),
        test_n15_000: ("15,000", "a"),
        test_n16_000: ("16,000", "a"),
        test_n17_000: ("17,000", "a"),
        test_n18_000: ("18,000", "an"),
        test_n19_000: ("19,000", "a"),
        test_n20_000: ("20,000", "a"),
        // numbers - decimal point
        test_n0_5: ("0.5", "a"),
        test_n1_5: ("1.5", "a"),
        test_n2_5: ("2.5", "a"),
        test_n3_5: ("3.5", "a"),
        test_n4_5: ("4.5", "a"),
        test_n5_5: ("5.5", "a"),
        test_n6_5: ("6.5", "a"),
        test_n7_5: ("7.5", "a"),
        test_n8_5: ("8.5", "an"),
        test_n9_5: ("9.5", "a"),
        test_n10_5: ("10.5", "a"),
        test_n11_5: ("11.5", "an"),
        test_n12_5: ("12.5", "a"),
        test_n13_5: ("13.5", "a"),
        test_n14_5: ("14.5", "a"),
        test_n15_5: ("15.5", "a"),
        test_n16_5: ("16.5", "a"),
        test_n17_5: ("17.5", "a"),
        test_n18_5: ("18.5", "an"),
        test_n19_5: ("19.5", "a"),
        test_n20_5: ("20.5", "a"),
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
        test_colloquial_ac4b: ("NATO", "an"),
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
        // numbers - years: with colloquial on, they are as read out loud: "an eighteen hundred"
        test_colloquial_ny1000: ("1000", "a"), // 'ten hundred'
        test_colloquial_ny1100: ("1100", "an"), // 'eleven hundred'
        test_colloquial_ny1800: ("1800", "an"),
        test_colloquial_ny1892: ("1892", "an"),
        // single letters - irregular
        test_single_irregular_u: ("u", "a"),
        test_single_irregular_f: ("f", "an"),
        test_single_irregular_h: ("h", "an"),
        test_single_irregular_l: ("l", "an"),
        test_single_irregular_m: ("m", "an"),
        test_single_irregular_n: ("n","an"),
        test_single_irregular_r: ("r", "an"),
        test_single_irregular_s: ("s", "an"),
        test_single_irregular_x: ("x", "an"),
        // single letters - regular
        test_single_regular_a: ("a", "an"),
        test_single_regular_b: ("b", "a"),
    }
}
