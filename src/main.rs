use std::env;

mod word_ladder;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args)
}

