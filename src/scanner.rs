use std::iter::Peekable;

#[derive(PartialEq, Debug)]
pub enum Token {
    LeftBrace,
    RightBrace,
    Colon,
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Comma,
    LeftBracket,
    RightBracket,
}

pub fn tokenize(input: &str) -> Option<Vec<Token>> {
    let mut iter = input.chars().peekable();
    let mut tokens = Vec::new();
    while let Some(c) = iter.next() {
        match c {
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            ':' => tokens.push(Token::Colon),
            ',' => tokens.push(Token::Comma),
            '[' => tokens.push(Token::LeftBracket),
            ']' => tokens.push(Token::RightBracket),
            '"' => tokens.push(tokenize_string(&mut iter)?),
            't' => tokens.push(tokenize_keyword(&mut iter, "true", Token::Boolean(true))?),
            'f' => tokens.push(tokenize_keyword(&mut iter, "false", Token::Boolean(false))?),
            'n' => tokens.push(tokenize_keyword(&mut iter, "null", Token::Null)?),
            c if c.is_ascii_digit() || c == '-' => tokens.push(tokenize_number(&mut iter, c)?),
            c if c.is_whitespace() => continue,
            _ => return None,
        }
    }

    Some(tokens)
}

fn tokenize_string(iter: &mut impl Iterator<Item = char>) -> Option<Token> {
    let mut str = String::new();
    while let Some(c) = iter.next() {
        match c {
            '"' => return Some(Token::String(str)),
            '\\' => {
                if let Some(escaped) = iter.next() {
                    str.push(match escaped {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '"' => '"',
                        _ => return None,
                    });
                }
            }
            _ => str.push(c),
        }
    }
    None
}

fn tokenize_number<I: Iterator<Item = char>>(
    iter: &mut Peekable<I>,
    first_digit: char,
) -> Option<Token> {
    let mut str = String::from(first_digit);

    while let Some(c) = iter.peek() {
        if !c.is_ascii_digit() && *c != '.' && *c != 'e' && *c != 'E' {
            break;
        }

        str.push(iter.next()?)
    }

    str.parse::<f64>().ok().map(Token::Number)
}

fn tokenize_keyword(
    iter: &mut impl Iterator<Item = char>,
    expected: &str,
    token: Token,
) -> Option<Token> {
    let mut chars = expected.chars();
    chars.next()?;
    if chars.all(|c| iter.next() == Some(c)) {
        return Some(token);
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::scanner::{tokenize, Token};

    #[test]
    fn test_empty_object() {
        let input = "{}";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens, vec![Token::LeftBrace, Token::RightBrace]);
    }

    #[test]
    fn test_empty_array() {
        let input = "[]";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens, vec![Token::LeftBracket, Token::RightBracket]);
    }

    #[test]
    fn test_simple_string() {
        let input = "\"hello\"";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens, vec![Token::String("hello".to_string())]);
    }

    #[test]
    fn test_boolean_true() {
        let input = "true";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens, vec![Token::Boolean(true)]);
    }

    #[test]
    fn test_boolean_false() {
        let input = "false";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens, vec![Token::Boolean(false)]);
    }

    #[test]
    fn test_null() {
        let input = "null";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens, vec![Token::Null]);
    }

    #[test]
    fn test_number() {
        let input = "42.5";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens, vec![Token::Number(42.5)]);
    }

    #[test]
    fn test_complex_json() {
        let input = "{\"key\": 123, \"flag\": true}";
        let tokens = tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LeftBrace,
                Token::String("key".to_string()),
                Token::Colon,
                Token::Number(123.0),
                Token::Comma,
                Token::String("flag".to_string()),
                Token::Colon,
                Token::Boolean(true),
                Token::RightBrace,
            ]
        );
    }
}
