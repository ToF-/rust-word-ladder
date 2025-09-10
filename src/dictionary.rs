use std::collections::HashSet;

pub type Dictionary = HashSet<String>;

pub fn new_dictionary(words: Vec<&str>) -> Dictionary {
        let mut result = Dictionary::new();
        for word in words {
            let _ = result.insert(word.to_string());
        };
        result
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loading_an_external_file() {
    }
}
