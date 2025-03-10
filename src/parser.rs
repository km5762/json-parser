use std::{collections::HashMap, iter::Peekable};

use crate::{scanner::Token, Object, Value};

pub fn parse(tokens: &[Token]) -> Option<Value> {
    let mut iter = tokens.iter().peekable();
    parse_value(&mut iter)
}

fn parse_key_value_pair<'a>(
    iter: &mut Peekable<impl Iterator<Item = &'a Token>>,
) -> Option<(String, Value)> {
    match iter.next() {
        Some(Token::String(key)) => {
            iter.next();
            Some((key.to_owned(), parse_value(iter)?))
        }
        _ => None,
    }
}

fn parse_value<'a>(iter: &mut Peekable<impl Iterator<Item = &'a Token>>) -> Option<Value> {
    if let Some(t) = iter.next() {
        match t {
            Token::Number(n) => Some(Value::Number(*n)),
            Token::String(s) => Some(Value::String(s.to_owned())),
            Token::Null => Some(Value::Null),
            Token::Boolean(b) => Some(Value::Boolean(*b)),
            Token::LeftBracket => Some(Value::Array(parse_array(iter)?)),
            Token::LeftBrace => Some(Value::Object(parse_object(iter)?)),
            _ => None,
        }
    } else {
        None
    }
}

fn parse_object<'a>(iter: &mut Peekable<impl Iterator<Item = &'a Token>>) -> Option<Object> {
    let mut object = Object::new();

    if **iter.peek()? == Token::RightBrace {
        return Some(object);
    }

    loop {
        let (key, value) = parse_key_value_pair(iter)?;
        object.insert(key, value);

        match iter.peek() {
            Some(Token::Comma) => {
                iter.next();
            }
            Some(Token::RightBrace) => {
                iter.next();
                break;
            }
            _ => return None,
        }
    }

    Some(object)
}

fn parse_array<'a>(iter: &mut Peekable<impl Iterator<Item = &'a Token>>) -> Option<Vec<Value>> {
    let mut array = Vec::new();

    if **iter.peek()? == Token::RightBracket {
        return Some(array);
    }

    loop {
        let v = parse_value(iter)?;
        array.push(v);

        match iter.peek() {
            Some(Token::Comma) => {
                iter.next();
            }
            Some(Token::RightBracket) => {
                iter.next();
                break;
            }
            _ => return None,
        }
    }

    Some(array)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::Token;

    #[test]
    fn test_parse_null() {
        let tokens = vec![Token::Null];
        assert_eq!(parse(&tokens), Some(Value::Null));
    }

    #[test]
    fn test_parse_boolean() {
        let tokens = vec![Token::Boolean(true)];
        assert_eq!(parse(&tokens), Some(Value::Boolean(true)));
    }

    #[test]
    fn test_parse_number() {
        let tokens = vec![Token::Number(42.0)];
        assert_eq!(parse(&tokens), Some(Value::Number(42.0)));
    }

    #[test]
    fn test_parse_string() {
        let tokens = vec![Token::String("hello".to_string())];
        assert_eq!(parse(&tokens), Some(Value::String("hello".to_string())));
    }

    #[test]
    fn test_parse_empty_array() {
        let tokens = vec![Token::LeftBracket, Token::RightBracket];
        assert_eq!(parse(&tokens), Some(Value::Array(vec![])));
    }

    #[test]
    fn test_parse_array_with_values() {
        let tokens = vec![
            Token::LeftBracket,
            Token::Number(1.0),
            Token::Comma,
            Token::Number(2.0),
            Token::Comma,
            Token::Number(3.0),
            Token::RightBracket,
        ];
        assert_eq!(
            parse(&tokens),
            Some(Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0)
            ]))
        );
    }

    #[test]
    fn test_parse_empty_object() {
        let tokens = vec![Token::LeftBrace, Token::RightBrace];
        assert_eq!(parse(&tokens), Some(Value::Object(Object::new())));
    }

    #[test]
    fn test_parse_object_with_values() {
        let tokens = vec![
            Token::LeftBrace,
            Token::String("key".to_string()),
            Token::Colon,
            Token::Number(42.0),
            Token::RightBrace,
        ];
        let mut expected = Object::new();
        expected.insert("key".to_string(), Value::Number(42.0));
        assert_eq!(parse(&tokens), Some(Value::Object(expected)));
    }

    #[test]
    fn test_parse_nested_object() {
        let tokens = vec![
            Token::LeftBrace,
            Token::String("outer".to_string()),
            Token::Colon,
            Token::LeftBrace,
            Token::String("inner".to_string()),
            Token::Colon,
            Token::Number(1.0),
            Token::RightBrace,
            Token::RightBrace,
        ];
        let mut inner = Object::new();
        inner.insert("inner".to_string(), Value::Number(1.0));
        let mut outer = Object::new();
        outer.insert("outer".to_string(), Value::Object(inner));
        assert_eq!(parse(&tokens), Some(Value::Object(outer)));
    }

    #[test]
    fn test_parse_nested_array() {
        let tokens = vec![
            Token::LeftBracket,
            Token::LeftBracket,
            Token::Number(1.0),
            Token::Comma,
            Token::Number(2.0),
            Token::RightBracket,
            Token::Comma,
            Token::LeftBracket,
            Token::Number(3.0),
            Token::RightBracket,
            Token::RightBracket,
        ];
        assert_eq!(
            parse(&tokens),
            Some(Value::Array(vec![
                Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]),
                Value::Array(vec![Value::Number(3.0)])
            ]))
        );
    }

    #[test]
    fn test_parse_mixed_nested_structure() {
        let tokens = vec![
            Token::LeftBrace,
            Token::String("array".to_string()),
            Token::Colon,
            Token::LeftBracket,
            Token::Number(1.0),
            Token::Comma,
            Token::String("text".to_string()),
            Token::Comma,
            Token::Null,
            Token::RightBracket,
            Token::Comma,
            Token::String("object".to_string()),
            Token::Colon,
            Token::LeftBrace,
            Token::String("key".to_string()),
            Token::Colon,
            Token::Boolean(false),
            Token::RightBrace,
            Token::RightBrace,
        ];
        let mut obj = Object::new();
        obj.insert(
            "array".to_string(),
            Value::Array(vec![
                Value::Number(1.0),
                Value::String("text".to_string()),
                Value::Null,
            ]),
        );
        let mut inner_obj = Object::new();
        inner_obj.insert("key".to_string(), Value::Boolean(false));
        obj.insert("object".to_string(), Value::Object(inner_obj));

        assert_eq!(parse(&tokens), Some(Value::Object(obj)));
    }

    #[test]
    fn test_parse_trailing_comma_array() {
        let tokens = vec![
            Token::LeftBracket,
            Token::Number(1.0),
            Token::Comma,
            Token::RightBracket,
        ];
        assert_eq!(parse(&tokens), None);
    }

    #[test]
    fn test_parse_trailing_comma_object() {
        let tokens = vec![
            Token::LeftBrace,
            Token::String("key".to_string()),
            Token::Colon,
            Token::Number(42.0),
            Token::Comma,
            Token::RightBrace,
        ];
        assert_eq!(parse(&tokens), None);
    }

    #[test]
    fn test_parse_unclosed_array() {
        let tokens = vec![Token::LeftBracket, Token::Number(1.0)];
        assert_eq!(parse(&tokens), None);
    }

    #[test]
    fn test_parse_unclosed_object() {
        let tokens = vec![
            Token::LeftBrace,
            Token::String("key".to_string()),
            Token::Colon,
            Token::Number(42.0),
        ];
        assert_eq!(parse(&tokens), None);
    }

    #[test]
    fn test_parse_unexpected_token() {
        let tokens = vec![Token::Comma];
        assert_eq!(parse(&tokens), None);
    }
}
