use std::process::exit; 
use std::env;
use crate::word_ladder::{word_ladder};
use crate::dictionary::{Dictionary, new_dictionary, dictionary_from_file};

mod word_ladder;
mod dictionary;
mod error;

fn main() {
    let args: Vec<String> = env::args().collect();
    match dictionary_from_file(&args[1]) {
        Ok(dictionary) => {
            match word_ladder(&args[2],&args[3], dictionary) {
                Ok(ladder) => {
                    println!("{:?}", ladder);
                    exit(0)
                },
                Err(e) => {
                    eprintln!("{}",e);
                    exit(2);
                },
            }
        },
        Err(e) => {
            eprintln!("{}",e);
            exit(1);
        }
    }
}

