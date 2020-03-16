use crate::string_helper;

use crate::options::Options;

use crate::rules;

// Traits required for the unit test assertions:
#[derive(Debug, PartialEq)]
pub enum Is {
    An,
    A,
    None,
}

pub fn a_or_an_capitalized_to_match(is_an: Is, first_word: &str) -> &'static str {
    let is_title_case = string_helper::is_title_case(first_word);

    match is_an {
        Is::An => {
            if is_title_case {
                return "An";
            }

            return "an";
        }
        Is::A => {
            if is_title_case {
                return "A";
            }

            "a"
        }
        _ => "",
    }
}

pub fn is_an_options_bool(word: &str, options: &Options) -> bool {
    let word = string_helper::get_first_word(word);

    let word_lower = word.to_lowercase();

    if rules::rule_numbers::is_number(word) {
        return rules::rule_numbers::is_an_for_number(word, options);
    }

    let is_an_result = is_naively_an(&word_lower);

    if rules::rule_acronyms::is_acronym(word) {
        return rules::rule_acronyms::is_an_for_acronym(word);
    }

    if rules::rule_exceptions::is_exception(&word_lower)
        || rules::rule_exceptions::is_exception_after_strip(&word_lower)
    {
        return !is_an_result;
    }

    is_an_result
}

fn is_naively_an(word: &str) -> bool {
    "aeiou".contains(string_helper::get_first_letter(word))
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
}
