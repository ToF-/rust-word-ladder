use std::env;
use crate::word_ladder::{Dictionary, new_dictionary, word_ladder};

mod word_ladder;
mod error;

fn main() {
    println!("{:?}", word_ladder("dog","cot",new_dictionary(vec!["dog","cog","cot"])));
    }

