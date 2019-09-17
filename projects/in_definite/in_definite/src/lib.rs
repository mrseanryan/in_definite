use regex::Regex;

pub fn get_a_or_an(word: &str) -> &str
{
    let mut is_an = is_naively_an(word);

    if is_exception(word)
    {
        is_an = !is_an;
    }

    if is_acronym(word)
    {
        return a_or_an_for_acronym(word);
    }

    if is_an
    {
        return "an"
    }
    else 
    {
     return "a";
    }
}

fn is_naively_an(word: &str) -> bool
{
    let first_letter = word.to_lowercase().chars().next().unwrap();

    if "aeiou".contains(first_letter) 
    {
        return true;
    }

    false
}

fn is_exception(word: &str) -> bool
{
    // ref: https://github.com/tandrewnichols/indefinite/blob/master/lib/irregular-words.js

    let exceptions = [
        // Nouns: eu like y
        "eunuch", "eucalyptus", "eugenics", "eulogy", "euphemism", "euphony", "euphoria", "eureka",

        // Adjectives: eu like y
        "european", "euphemistic", "euphonic", "euphoric",

        // Adverbs: eu like y
        "euphemistically", "euphonically", "euphorically",

        // Nouns: silent h
        "heir", "heiress", "herb", "homage", "honesty", "honor", "honour", "hour",

        // Adjectives: silent h
        "honest", "honorous",

        // Adverbs: silent h
        "honestly", "hourly",

        // Nouns: o like w
        "one", "ouija",

        // Adjectives: o like w
        "once",

        // Adverbs: o like w

        // Nouns: u like y
        "ubiquity", "udometer", "ufo", "uke", "ukelele", "ululate", "unicorn", "unicycle", "uniform",
        "unify", "union", "unison", "unit", "unity", "universe", "university", "upas", "ural", "uranium",
        "urea", "ureter", "urethra", "urine", "urology", "urus", "usage", "use", "user", "usual", "usurp",
        "usury", "utensil", "uterus", "utility", "utopia", "utricle", "uvarovite", "uvea", "uvula",

        // Adjectives: u like y
        "ubiquitous", "ugandan", "ukrainian", "unanimous", "unicameral", "unified", "unique", "unisex",
        "universal", "urinal", "urological", "useful", "useless", "usurious", "usurped", "utilitarian",
        "utopic",

        // Adverbs: u like y
        "ubiquitously", "unanimously", "unicamerally", "uniquely", "universally", "urologically", "usefully", "uselessly", "usuriously",

        // Nouns: y like i
        "yttria", "yggdrasil", "ylem", "yperite", "ytterbia", "ytterbium", "yttrium",

        // Adjectives: y like i
        "ytterbous", "ytterbic", "yttric"
        ];

    return exceptions.contains(&word);
}

fn is_acronym(word: &str) -> bool
{
    let re = Regex::new(r"^[A-Z]+$").unwrap();
    return re.is_match(word);
}

// ref: https://github.com/tandrewnichols/indefinite/blob/master/lib/rules/acronyms.js
fn a_or_an_for_acronym(word: &str) -> &str
{
  let is_irregular = is_irregular_acronym(word);
  let initial_vowel = starts_with_vowel(word);
  /*
   * If it starts with U: "a"
   * If it starts with any other vowel: "an"
   * If it starts with F, H, L, M, N, R, S, or X: "an"
   * If it starts with any other consonant: "a"
   */
  let mut article = "an";
  if both_or_neither(initial_vowel, is_irregular)
  {
      article = "a";
  }
  return article;
}

fn both_or_neither(a: bool, b: bool) -> bool
{
    return a && b || !a && !b;
}

fn is_irregular_acronym(word: &str) -> bool
{
    let re = Regex::new(r"^[UFHLMNRSX]").unwrap();
    return re.is_match(word);
}

fn starts_with_vowel(word: &str) -> bool
{
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
        assert_eq!(
            "an",
            get_a_or_an("antelope")
        );
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
        test_0: ("alien", "an"),
        test_1: ("antelope", "an"), 
        test_2: ("EU", "an"),
        test_3: ("FIFA", "an"),
        test_4: ("herb", "an"), // USA not UK
        test_5: ("hotel", "a"),
        test_6: ("MIA", "an"),
        test_7: ("MNM", "an"),
        test_8: ("UFO", "a"),
        test_9: ("UN", "a"),
        test_10: ("umbrella", "an"),
        test_11: ("user", "a"),
    }
}