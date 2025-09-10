use std::error::Error;
use crate::error::WordNotFoundError;
use std::collections::{HashMap, HashSet};
use crate::dictionary::{Dictionary, new_dictionary};

pub fn greeting(name: Option<&str>) -> String {
    match name {
        Some(s) => format!("hello, {}!", s),
        None => String::from("hello, world!"),
    }
}

fn neighbor(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        false
    } else {
        a.chars().zip(b.chars())
            .filter(|(a_char, b_char)| a_char != b_char)
            .count() == 1
    }
}

fn follow_ladder(ladder: HashMap<String, Option<String>>, start: &str, end: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![end.to_string()];
    let mut rung: Option<String> = Some(end.to_string());
    while let Some(target) = ladder.get(&rung.clone().unwrap()) {
        match target {
            Some(word) => { 
                result.insert(0, word.to_string());
                rung = Some(word.to_string());
                if word == start {
                    return result
                };
            },
            None => {
                return vec![]
            }
        }
    };
    return vec![]
}
pub fn word_ladder(start: &str, end: &str, mut dictionary: Dictionary) -> Result<Vec<String>,WordNotFoundError> {
    if ! dictionary.contains(start) {
        Err(WordNotFoundError { word: start.to_string() })
    } else if ! dictionary.contains(end) {
        Err(WordNotFoundError { word: end.to_string() })
    } else {
        let mut queue: Vec<String> = vec![];
        let mut ladder: HashMap<String,Option<String>> = HashMap::new();
        queue.push(start.to_string());
        ladder.insert(start.to_string(), None);
        while !queue.is_empty() {
            let rung = queue.pop().unwrap();
            if rung == end {
                return Ok(follow_ladder(ladder, start, end))
            } else {
                let neighbors = dictionary.clone()
                    .into_iter()
                    .filter(|w| neighbor(&rung, w));
                for next in neighbors {
                    dictionary.remove(&next);
                    ladder.insert(next.clone(), Some(rung.to_string()));
                    queue.push(next.clone());
                }
            }
        };
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
        assert_eq!(word_ladder("dog","fog",new_dictionary(vec!["dog","fog"])).unwrap(),
            [String::from("dog"),String::from("fog")]);
    }
    #[test]
    fn another_word_ladder_with_one_rung() {
        assert_eq!(word_ladder("fog","dog",new_dictionary(vec!["dog","fog"])).unwrap(),
            [String::from("fog"),String::from("dog")]);
    }
    #[test]
    fn word_ladder_with_two_rungs() {
        assert_eq!(word_ladder("dog","cot",new_dictionary(vec!["dog","cog","cot"])).unwrap(),
            [String::from("dog"),String::from("cog"), String::from("cot")]);
    }
}
