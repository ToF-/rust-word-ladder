fn greeting(name: Option<&str>) -> String {
    String::from("hello, world!")
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting_with_none_argument_is_just_hello_word() {
        assert_eq!(greeting(None), "hello, world!")
    }
}
