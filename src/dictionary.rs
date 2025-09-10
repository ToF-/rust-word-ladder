use std::collections::HashSet;
use std::io::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub type Dictionary = HashSet<String>;

pub fn new_dictionary(words: Vec<&str>) -> Dictionary {
        let mut result = Dictionary::new();
        for word in words {
            let _ = result.insert(word.to_string());
        };
        result
    }

pub fn dictionary_from_file(file_name: &str) -> Result<Dictionary,Error> {
    let mut result: Dictionary = HashSet::new();
    let result = File::open(file_name).and_then(|file| {
        let reader = io::BufReader::new(file);
        let lines: Vec<String> = reader
            .lines()
            .collect::<Result<_, _>>().expect("can't read file");
            Ok(lines.into_iter().collect())
        });
    match result {
        Ok(r) => Ok(r),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loading_an_external_file() {
        let dictionary = dictionary_from_file("testdata/ukwords.txt").expect("can't file file");
        assert_eq!(dictionary.len(), 57046);
    }
}
