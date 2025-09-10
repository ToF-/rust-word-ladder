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
    let file = File::open(file_name)?;
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()?;

    Ok(lines.into_iter().collect())
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
