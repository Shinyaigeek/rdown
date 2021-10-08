pub fn tokenize(source: &str) -> Vec<&str> {
    let mut tokens = vec![];

    for char in source.chars() {}

    tokens
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenize_works() {
        let tokens: Vec<&str> = vec![];
        assert_eq!(tokenize(""), tokens);
    }
}
