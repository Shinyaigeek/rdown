use crate::parser::tokenizer::token::Token;

pub fn tokenize<'a>(source: &str) -> Vec<Token<'a>> {
    let mut tokens: Vec<Token<'a>> = vec![];
    let mut is_escaping = false;
    let mut is_breaked = true;

    for c in source.chars() {
        if c == '\n' {
            tokens.push(Token::Break);
            is_breaked = true;
            continue;
        }
        is_breaked = false;
        if c == '\\' {
            is_escaping = true;
            continue;
        }

        if c == ' ' {
            is_escaping = false;
        }
    }

    tokens
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenize_works_with_empty_source() {
        let tokens: Vec<Token<'static>> = vec![];
        assert_eq!(tokenize(""), tokens);
    }
}
