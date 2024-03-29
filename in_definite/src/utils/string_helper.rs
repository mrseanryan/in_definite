pub fn get_first_word(word: &str) -> &str {
    let word = word.trim();

    let words: Vec<&str> = word.split(|c: char| " ,.-;:'".contains(c)).collect();

    words[0]
}

pub fn get_first_letter(word: &str) -> char {
    word.chars().next().unwrap()
}

pub fn is_title_case(first_word: &str) -> bool {
    let mut chars = first_word.chars();
    chars.next().unwrap().is_uppercase() && chars.all(|ch| ch.is_lowercase())
}

pub fn strip_end<'s>(word: &'s str, ending: &str) -> &'s str {
    if word.ends_with(ending) {
        let stripped = &word[..(word.len() - ending.len())];
        if stripped.len() > 1 {
            return stripped;
        }
    }

    word
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_first_word_test() {
        assert_eq!("one", get_first_word("one two"));
        assert_eq!("one", get_first_word("one two three"));
        assert_eq!("one", get_first_word("one-two three"));
        assert_eq!("heir", get_first_word("heir's"));
    }

    #[test]
    fn is_title_case_test() {
        assert_eq!(false, is_title_case("one"));
        assert_eq!(true, is_title_case("Two"));
        assert_eq!(false, is_title_case("THree"));
        assert_eq!(false, is_title_case("FOUR"));
    }

    #[test]
    fn strip_end_test() {
        assert_eq!("one", strip_end("ones", "s"));
        assert_eq!("heir", strip_end("heir's", "'s"));
        assert_eq!("hour", strip_end("houred", "ed"));
        assert_eq!("hour", strip_end("hourly", "ly"));
        assert_eq!("hour", strip_end("hour's", "'s"));
        // Do NOT strip work like 'red' down to one letter like 'r' as that would incorrectly handle the word as a single-letter exception:
        assert_eq!("red", strip_end("red", "'red"));
    }
}
