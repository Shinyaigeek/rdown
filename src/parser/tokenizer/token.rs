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
        // * because first sharp have been eaten
        let mut sharp_cnt: u8 = 1;
        let mut heading_text = String::from("");
        let mut spaced = false;

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
                spaced = true;
                source.next();
            } else if c == &'\n' {
                break;
            } else {
                heading_text.push(*c);
                source.next();
            }
        }

        if sharp_cnt <= 6 {
            // let heading_text: &'a str = &heading_text;
            Self::Heading((sharp_cnt, heading_text))
        } else {
            panic!("TODO");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn handle_sharp_works_with_simple_headings() {
        let mut source = "## hoge".chars().peekable();
        source.next();
        let token = Token::handle_sharp(&mut source);
        assert_eq!(token, Token::Heading((2, "hoge".to_string())));

        let mut source = "### bar".chars().peekable();
        source.next();
        let token = Token::handle_sharp(&mut source);
        assert_eq!(token, Token::Heading((3, "bar".to_string())));
    }
}
