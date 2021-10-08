use crate::parser::tokenizer::token::Token;

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut is_breaked = true;

    let mut source = source.chars().peekable();

    loop {
        if source.peek().is_none() {
            break;
        }

        // * because peekable
        let c = source.peek().unwrap();
        if c == &'\n' {
            tokens.push(Token::Break);
            is_breaked = true;
            continue;
        }

        if c == &'#' && is_breaked {
            tokens.push(Token::handle_sharp(&mut source));
            break;
        }

        if c == &'`' {
            tokens.push(Token::handle_back_quote(&mut source));
            break;
        }

        if c == &'>' {
            tokens.push(Token::handle_greater(&mut source));
            break;
        }
    }

    tokens
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenize_works_with_empty_source() {
        let tokens: Vec<Token> = vec![];
        assert_eq!(tokenize(""), tokens);
    }
}
