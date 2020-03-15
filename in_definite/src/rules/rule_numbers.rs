use crate::options::Options;
use crate::string_helper;

// ref: https://github.com/tandrewnichols/indefinite/blob/master/lib/rules/numbers.js
pub fn is_number(word: &str) -> bool {
    string_helper::get_first_letter(word).is_numeric()
}

pub fn is_an_for_number(word: &str, options: &Options) -> bool {
    let mut is_an = false;

    // If the number starts with 11 or 18 and is of length 4,
    // the pronunciation is ambiguous so check opts.numbers to see
    // how to render it. Otherwise, if it starts with 11 or 18
    // and has 2, 5, 8, 11, etc. digits, use 'an.' Finally, if it
    // starts with an 8, use 'an.' For everything else, use 'a.'
    let starts_with_11_or_18 = word.starts_with("11") || word.starts_with("18");
    if starts_with_11_or_18 && word.len() == 4 {
        is_an = options.are_numbers_colloquial;
    } else if starts_with_11_or_18 && (word.len() - 2) % 3 == 0 {
        is_an = true;
    } else if word.starts_with('8') {
        is_an = true;
    }

    is_an
}

