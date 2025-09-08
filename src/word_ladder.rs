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
    } else {
        Err(WordNotFoundError { word: end.to_string() })
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
        let Err(e) = word_ladder("foo","bar", HashSet::new()) else { todo!() } ;
        assert_eq!(e, WordNotFoundError { word: String::from("foo") })
    }

    #[test]
    fn word_ladder_with_non_existing_end_word_results_in_error() {
        let mut dictionary: HashSet<String> = HashSet::new();
        let _ = dictionary.insert(String::from("foo"));
        let Err(e) = word_ladder("foo","bar", dictionary) else { todo!() } ;
        assert_eq!(e, WordNotFoundError { word: String::from("bar") })
    }
}
