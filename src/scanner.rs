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

pub fn tokenize(input: String) -> Vec<Token> {
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
            '"' => tokens.push(tokenize_string(&mut iter)),
            't' => tokens.push(tokenize_keyword(&mut iter, "true", Token::Boolean(true)).unwrap()),
            'f' => {
                tokens.push(tokenize_keyword(&mut iter, "false", Token::Boolean(false)).unwrap())
            }
            'n' => tokens.push(tokenize_keyword(&mut iter, "null", Token::Null).unwrap()),
            c if c.is_ascii_digit() => tokens.push(tokenize_number(&mut iter, c).unwrap()),
            c if c.is_whitespace() => continue,
            _ => panic!("uh oh"),
        }
    }

    tokens
}

fn tokenize_string(iter: &mut impl Iterator<Item = char>) -> Token {
    let mut str = String::new();
    while let Some(c) = iter.next() {
        match c {
            '"' => break, // Closing quote
            '\\' => {
                if let Some(escaped) = iter.next() {
                    str.push(match escaped {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '"' => '"',
                        _ => panic!("uh oh!"),
                    });
                }
            }
            _ => str.push(c),
        }
    }
    Token::String(str)
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
