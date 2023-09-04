use crate::string_helper;

pub fn is_exception_after_strip(word_lower: &str) -> bool {
    // into_iter():
    // book: Rust 2018 says: If we want to create an iterator that takes ownership of v1 and returns owned values, we can call into_iter instead of iter.
    ENDINGS
        .into_iter()
        .map(|ending| string_helper::strip_end(&word_lower, ending))
        .any(|stripped| is_exception(stripped))
}

const ENDINGS: &[&str] = &["s", "es", "ed", "ly"];

pub fn is_exception(word: &str) -> bool {
    // ref: https://github.com/tandrewnichols/indefinite/blob/master/lib/irregular-words.js

    EXCEPTIONS.contains(&word)
}

const EXCEPTIONS: &[&str] = &[
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
    "usurper",
    "usury",
    "utah",
    "utahn",
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
    // Single letters
    "u",
    "f",
    "h",
    "l",
    "m",
    "n",
    "r",
    "s",
    "x",
];
