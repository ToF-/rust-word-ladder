use std::error::Error;
use crate::error::WordNotFoundError;
use std::collections::HashSet;

pub type Dictionary = HashSet<String>;

pub fn greeting(name: Option<&str>) -> String {
    match name {
        Some(s) => format!("hello, {}!", s),
        None => String::from("hello, world!"),
    }
}

fn neighbor(a: &str, b: &str) -> bool {
    let mut count: usize = 0;
    for (i, char_a) in a.chars().enumerate() {
        match b.chars().nth(i) {
            None => return false,
            Some(char_b) => {
                if char_b != char_a {
                    count+= 1;
                    if count > 1 {
                        return false
                    }
                };
            },
        }
    };
    count == 1
}

pub fn word_ladder(start: &str, end: &str, dictionary: Dictionary) -> Result<Vec<String>,WordNotFoundError> {
    if ! dictionary.contains(start) {
        Err(WordNotFoundError { word: start.to_string() })
    } else if ! dictionary.contains(end) {
        Err(WordNotFoundError { word: end.to_string() })
    } else {
        if neighbor(start, end) {
            Ok(vec![String::from(start), String::from(end)])
        } else {
            Ok(vec![])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_dictionary(words: Vec<&str>) -> Dictionary {
        let mut result = Dictionary::new();
        for word in words {
            let _ = result.insert(word.to_string());
        };
        result
    }
    #[test]
    fn test_greeting_with_none_argument_is_just_hello_world() {
        assert_eq!(greeting(None), "hello, world!")
    }

    #[test]
    fn test_greeting_with_some_name_is_hello_name() {
        assert_eq!(greeting(Some("ToF")), "hello, ToF!")
    }

    #[test]
    fn word_ladder_with_non_existing_start_word_results_in_error() {
        let dictionary: Dictionary = new_dictionary(vec![]);
        let result = word_ladder("foo","bar", dictionary);
        assert_eq!(result.is_err(), true);
        assert_eq!(result.unwrap_err(), WordNotFoundError { word: String::from("foo") })
    }

    #[test]
    fn word_ladder_with_non_existing_end_word_results_in_error() {
        let mut dictionary: Dictionary = new_dictionary(vec!["foo"]);
        let result = word_ladder("foo","bar", dictionary);
        assert_eq!(result.is_err(), true);
        assert_eq!(result.unwrap_err(), WordNotFoundError { word: String::from("bar") })
    }

    #[test]
    fn no_word_ladder_found_results_in_empty_list() {
        let mut dictionary: Dictionary = Dictionary::new();
        let _ = dictionary.insert(String::from("foo"));
        let _ = dictionary.insert(String::from("bar"));
        let result = word_ladder("foo","bar", dictionary);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn word_ladder_with_one_rung() {
        let mut dictionary: Dictionary = Dictionary::new();
        let _ = dictionary.insert(String::from("dog"));
        let _ = dictionary.insert(String::from("fog"));
        let result = word_ladder("dog","fog", dictionary);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), [String::from("dog"),String::from("fog")]);
    }
    #[test]
    fn another_word_ladder_with_one_rung() {
        let mut dictionary: Dictionary = Dictionary::new();
        let _ = dictionary.insert(String::from("dog"));
        let _ = dictionary.insert(String::from("fog"));
        let result = word_ladder("fog","dog", dictionary);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), [String::from("fog"),String::from("dog")]);
    }
}
