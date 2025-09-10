use std::env;
use crate::word_ladder::{word_ladder};
use crate::dictionary::{Dictionary, new_dictionary};

mod word_ladder;
mod dictionary;
mod error;

fn main() {
    println!("{:?}", word_ladder("dog","cot",new_dictionary(vec!["dog","cog","cot"])));
    }

