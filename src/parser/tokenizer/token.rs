use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Paragraph(String),
    Heading((u8, String)),
    // TODO
    Html(String),
    Code((String, String)),
    Blockquote(String),
    ThematicBreak,
    List(String),
    ListItem(String),
    Definition((String, String)),
    Emphasis(String),
    Strong(String),
    InlineCode(String),
    Break,
    Link((String, String)),
    Image((String, String)),
    LinkReference((String, String)),
    ImageReference((String, String)),
}

impl Token {
    pub fn handle_sharp(source: &mut Peekable<Chars>) -> Self {
        let mut sharp_cnt: u8 = 0;

        let mut rev_source = source.clone();

        loop {
            let c = source.peek();
            if c.is_none() {
                break;
            }

            // because c should be Some
            let c = c.unwrap();

            if c == &'#' {
                sharp_cnt += 1;
                source.next();
            } else if c == &' ' {
                source.next();
                break;
            } else {
                // iterator current position should be same
                return Self::handle_text(&mut rev_source);
            }
        }

        let heading_text = Self::handle_heading_text(source);

        if sharp_cnt <= 6 {
            Self::Heading((sharp_cnt, heading_text))
        } else {
            // iterator current position should be same
            return Self::handle_text(&mut rev_source);
        }
    }

    fn handle_heading_text(source: &mut Peekable<Chars>) -> String {
        let mut heading_text = "".to_string();
        loop {
            let c = source.peek();
            if c.is_none() {
                break;
            }

            // because c should be Some
            let c = c.unwrap();

            if c == &'\n' {
                break;
            } else {
                heading_text.push(*c);
                source.next();
            }
        }

        heading_text
    }

    pub fn handle_text(source: &mut Peekable<Chars>) -> Self {
        let mut text = String::from("");

        loop {
            let c = source.peek();
            if c.is_none() {
                break;
            }

            // because c should be Some
            let c = c.unwrap();

            if c == &'\n' {
                break;
            } else {
                text.push(*c);
                source.next();
            }
        }

        Self::Paragraph(text)
    }

    pub fn handle_back_quote(source: &mut Peekable<Chars>) -> Self {
        source.next();

        if source.peek().unwrap_or(&' ') == &'`' {
            source.next();
            if source.peek().unwrap_or(&' ') == &'`' {
                source.next();
                Self::handle_code(source)
            } else {
                Self::InlineCode(String::from(""))
            }
        } else {
            Self::handle_inline_code(source)
        }
    }

    fn handle_inline_code(source: &mut Peekable<Chars>) -> Self {
        let mut inline_code = String::from("");

        loop {
            if source.peek().is_none() {
                break;
            }

            // * because peekable
            let c = source.peek().unwrap();

            if c == &'\n' || c == &'`' {
                source.next();
                break;
            } else {
                inline_code.push(source.next().unwrap());
            }
        }

        Self::InlineCode(inline_code)
    }

    fn handle_code(source: &mut Peekable<Chars>) -> Self {
        let mut language = Self::handle_code_language(source);
        source.next();
        let mut code = String::from("");

        loop {
            let c = source.peek();

            if c.is_none() {
                break;
            }

            let c = c.unwrap();

            code.push(source.next().unwrap());
            if code.ends_with("\n```") {
                code.pop();
                code.pop();
                code.pop();
                code.pop();
                break;
            }
        }

        Self::Code((language, code))
    }

    fn handle_code_language(source: &mut Peekable<Chars>) -> String {
        let mut language = String::from("");
        loop {
            let c = source.peek();

            if c.is_none() {
                return "".to_string();
            }

            let c = c.unwrap();

            if c == &'\n' {
                return language;
            } else {
                language.push(source.next().unwrap());
            }
        }
    }

    pub fn handle_greater(source: &mut Peekable<Chars>) -> Self {
        source.next();

        let mut quote_text = String::from("");

        loop {
            let c = source.peek();

            if c.is_none() {
                break;
            }

            let c = c.unwrap();

            quote_text.push(source.next().unwrap());

            if quote_text.ends_with("\n\n") {
                quote_text.pop();
                quote_text.pop();
                break;
            }
        }

        Self::Blockquote(quote_text)
    }

    pub fn handle_left_brace(source: &mut Peekable<Chars>) -> Self {
        let mut rev_source = source.clone();
        source.next();
        let label = Self::handle_link_label(source);

        if label.is_none() {
            // current iterator position should be same
            return Self::handle_text(&mut rev_source);
        }

        let label = label.unwrap();

        source.next();

        let c = source.peek();

        if c.unwrap_or(&' ') == &'(' {
            source.next();
            let href = Self::handle_link_href(source);
            if href.is_none() {
                // current iterator position should be same
                return Self::handle_text(&mut rev_source);
            } else {
                source.next();
                let href = href.unwrap();
                Self::Link((label, href))
            }
        } else if c.unwrap_or(&' ') == &'[' {
            source.next();
            let identifier = Self::handle_link_label(source);
            if identifier.is_none() {
                // current iterator position should be same
                return Self::handle_text(&mut rev_source);
            } else {
                source.next();
                let identifier = identifier.unwrap();
                Self::LinkReference((label, identifier))
            }
        } else {
            // current iterator position should be same
            return Self::handle_text(&mut rev_source);
        }
    }

    fn handle_link_label(source: &mut Peekable<Chars>) -> Option<String> {
        let mut label = String::from("");
        let mut brace_diff = 0;

        loop {
            let c = source.peek();

            if c.is_none() {
                return None;
            }

            let c = c.unwrap();

            if c == &'\n' {
                return None;
            }

            if c == &']' && brace_diff == 0 {
                return Some(label);
            }

            if c == &'[' {
                brace_diff += 1;
            }
            if c == &']' {
                brace_diff -= 1;
            }

            label.push(source.next().unwrap())
        }
    }

    fn handle_link_href(source: &mut Peekable<Chars>) -> Option<String> {
        let mut href = String::from("");
        let mut brace_diff = 0;

        loop {
            let c = source.peek();

            if c.is_none() {
                return None;
            }

            let c = c.unwrap();

            if c == &'\n' {
                return None;
            }

            if c == &')' && brace_diff == 0 {
                return Some(href);
            }

            if c == &'(' {
                brace_diff += 1;
            }
            if c == &')' {
                brace_diff -= 1;
            }

            href.push(source.next().unwrap());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn handle_sharp_works_with_simple_headings() {
        let mut source = "## hoge".chars().peekable();
        let token = Token::handle_sharp(&mut source);
        assert_eq!(token, Token::Heading((2, "hoge".to_string())));

        let mut source = "### bar".chars().peekable();
        let token = Token::handle_sharp(&mut source);
        assert_eq!(token, Token::Heading((3, "bar".to_string())));

        let mut source = "###### fuga".chars().peekable();
        let token = Token::handle_sharp(&mut source);
        assert_eq!(token, Token::Heading((6, "fuga".to_string())));
    }

    #[test]
    fn more_than_7_sharp_should_be_text() {
        let mut source = "####### hoge
## hoge"
            .chars()
            .peekable();
        let token = Token::handle_sharp(&mut source);
        assert_eq!(token, Token::Paragraph("####### hoge".to_string()));
        source.next();
        let token = Token::handle_sharp(&mut source);
        assert_eq!(token, Token::Heading((2, "hoge".to_string())))
    }

    #[test]
    fn sharp_with_text_without_space_should_be_text() {
        let mut source = "##hoge".chars().peekable();
        let token = Token::handle_sharp(&mut source);
        assert_eq!(token, Token::Paragraph("##hoge".to_string()));
    }

    #[test]
    fn handle_text_with_siple_text() {
        let mut source = "asdf
hoge"
            .chars()
            .peekable();
        let token = Token::handle_text(&mut source);
        assert_eq!(token, Token::Paragraph("asdf".to_string()));
    }

    #[test]
    fn handle_inline_code_with_simple_text() {
        let mut source = "`hoge`".chars().peekable();
        let token = Token::handle_back_quote(&mut source);
        assert_eq!(token, Token::InlineCode("hoge".to_string()))
    }

    #[test]
    fn handle_inline_code_with_empty_text() {
        let mut source = "``".chars().peekable();
        let token = Token::handle_back_quote(&mut source);
        assert_eq!(token, Token::InlineCode("".to_string()))
    }

    #[test]
    fn handle__code_with_simple_text() {
        let mut source = "```javascript
const hoge = 1;
const bar = 2;
```"
        .chars()
        .peekable();
        let token = Token::handle_back_quote(&mut source);
        assert_eq!(
            token,
            Token::Code((
                "javascript".to_string(),
                "const hoge = 1;
const bar = 2;"
                    .to_string()
            ))
        )
    }

    #[test]
    fn handle_greater_with_simple_text() {
        let mut source = ">hoge".chars().peekable();

        let token = Token::handle_greater(&mut source);

        assert_eq!(token, Token::Blockquote("hoge".to_string()))
    }

    #[test]
    fn handle_greater_with_text_with_breakline() {
        let mut source = ">hoge
fuga

bar"
        .chars()
        .peekable();

        let token = Token::handle_greater(&mut source);

        assert_eq!(
            token,
            Token::Blockquote(
                "hoge
fuga"
                    .to_string()
            )
        )
    }

    #[test]
    fn parse_link_correctly() {
        let mut source = "[hoge](fuga)".chars().peekable();

        let token = Token::handle_left_brace(&mut source);

        assert_eq!(token, Token::Link(("hoge".to_string(), "fuga".to_string())))
    }
}
