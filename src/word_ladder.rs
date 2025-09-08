fn greeting(name: Option<&str>) -> String {
    match name {
        Some(s) => format!("hello, {}!", s),
        None => String::from("hello, world!"),
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
}
