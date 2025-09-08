use std::error::Error;
use crate::error::WordNotFoundError;
use std::collections::HashSet;

pub fn greeting(name: Option<&str>) -> String {
    match name {
        Some(s) => format!("hello, {}!", s),
        None => String::from("hello, world!"),
    }
}

pub fn word_ladder(start: &str, end: &str, dictionary: HashSet<String>) -> Result<Vec<String>,WordNotFoundError> {
    if ! dictionary.contains(start) {
        Err(WordNotFoundError { word: start.to_string() })
    } else if ! dictionary.contains(end) {
        Err(WordNotFoundError { word: end.to_string() })
    } else {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let dictionary: HashSet<String> = HashSet::new();
        let result = word_ladder("foo","bar", dictionary);
        assert_eq!(result.is_err(), true);
        assert_eq!(result.unwrap_err(), WordNotFoundError { word: String::from("foo") })
    }

    #[test]
    fn word_ladder_with_non_existing_end_word_results_in_error() {
        let mut dictionary: HashSet<String> = HashSet::new();
        let _ = dictionary.insert(String::from("foo"));
        let result = word_ladder("foo","bar", dictionary);
        assert_eq!(result.is_err(), true);
        assert_eq!(result.unwrap_err(), WordNotFoundError { word: String::from("bar") })
    }

    #[test]
    fn no_word_ladder_found_results_in_empty_list() {
        let mut dictionary: HashSet<String> = HashSet::new();
        let _ = dictionary.insert(String::from("foo"));
        let _ = dictionary.insert(String::from("bar"));
        let result = word_ladder("foo","bar", dictionary);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().len(), 0);
    }
}
