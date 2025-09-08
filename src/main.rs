use std::env;
use crate::word_ladder::greeting;

mod word_ladder;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("{}", greeting(Some(&args[1].clone())))
    } else {
        println!("{}", greeting(None))
    }
}

