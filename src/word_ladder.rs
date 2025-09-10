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
    assert!(a.len() == b.len());
    a.chars().zip(b.chars())
        .filter(|(a_char, b_char)| a_char != b_char)
        .count() == 1
}

pub fn word_ladder(start: &str, end: &str, dictionary: Dictionary) -> Result<Vec<String>,WordNotFoundError> {
    if ! dictionary.contains(start) {
        Err(WordNotFoundError { word: start.to_string() })
    } else if ! dictionary.contains(end) {
        Err(WordNotFoundError { word: end.to_string() })
    } else {
        let mut first: String = String::from("");
        let mut last: String = String::from("");
        let mut dict = dictionary.clone();
        match dict.take(start) {
            Some(word) => {
                let first = word.to_string();
                let neighbors: Dictionary = dict.extract_if(|w| neighbor(&first, w)).collect();
                match neighbors.iter().next() {
                    Some(word) => {
                        let last = word.to_string();
                        return Ok(vec![first, last])
                    },
                    None => {} ,
                };
            },
            None => {},
            }
            Ok(vec![])
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
        assert_eq!(word_ladder("foo", "bar", new_dictionary(vec![])).unwrap_err(),
            WordNotFoundError { word: String::from("foo") })
    }

    #[test]
    fn word_ladder_with_non_existing_end_word_results_in_error() {
        assert_eq!(word_ladder("foo","bar",new_dictionary(vec!["foo"])).unwrap_err(),
        WordNotFoundError { word: String::from("bar") })
    }

    #[test]
    fn no_word_ladder_found_results_in_empty_list() {
        assert_eq!(word_ladder("foo","bar",new_dictionary(vec!["foo","bar"])).unwrap().len(), 0);
    }

    #[test]
    fn word_ladder_with_one_rung() {
        let dictionary: Dictionary = new_dictionary(vec!["dog", "fog"]);
        assert_eq!(word_ladder("dog","fog",new_dictionary(vec!["dog","fog"])).unwrap(),
            [String::from("dog"),String::from("fog")]);
    }
    #[test]
    fn another_word_ladder_with_one_rung() {
        assert_eq!(word_ladder("fog","dog",new_dictionary(vec!["dog","fog"])).unwrap(),
            [String::from("fog"),String::from("dog")]);
    }
}
