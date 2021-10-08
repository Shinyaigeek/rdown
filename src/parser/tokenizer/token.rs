use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Paragraph(String),
    Heading((u8, String)),
    Html(String),
    Code(String),
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
                let nth = source.count();
                source.nth(nth - sharp_cnt as usize);
                return Self::handle_text(source);
            }
        }

        let heading_text = Self::handle_heading_text(source);

        if sharp_cnt <= 6 {
            Self::Heading((sharp_cnt, heading_text))
        } else {
            let nth = source.count();
            source.nth(nth - sharp_cnt as usize);
            return Self::handle_text(source);
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
}
