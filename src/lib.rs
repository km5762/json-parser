use std::collections::HashMap;

use parser::parse;
use scanner::tokenize;

mod parser;
mod scanner;

pub type Object = HashMap<String, Value>;

#[derive(PartialEq, Debug)]
pub enum Value {
    String(String),
    Array(Vec<Value>),
    Object(Object),
    Boolean(bool),
    Number(f64),
    Null,
}

impl Value {
    pub fn from_str(input: &str) -> Option<Self> {
        let tokens = tokenize(input)?;
        parse(&tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        assert_eq!(
            Value::from_str("\"hello\""),
            Some(Value::String("hello".to_string()))
        );
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(Value::from_str("42"), Some(Value::Number(42.0)));
        assert_eq!(Value::from_str("3.14"), Some(Value::Number(3.14)));
        assert_eq!(Value::from_str("-123"), Some(Value::Number(-123.0)));
        assert_eq!(Value::from_str("0.0"), Some(Value::Number(0.0)));
    }

    #[test]
    fn test_parse_boolean() {
        assert_eq!(Value::from_str("true"), Some(Value::Boolean(true)));
        assert_eq!(Value::from_str("false"), Some(Value::Boolean(false)));
    }

    #[test]
    fn test_parse_null() {
        assert_eq!(Value::from_str("null"), Some(Value::Null));
    }

    #[test]
    fn test_parse_array() {
        assert_eq!(
            Value::from_str("[1, 2, 3]"),
            Some(Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0)
            ]))
        );
        assert_eq!(
            Value::from_str("[\"a\", null, true]"),
            Some(Value::Array(vec![
                Value::String("a".to_string()),
                Value::Null,
                Value::Boolean(true)
            ]))
        );
    }

    #[test]
    fn test_parse_object() {
        let mut expected = Object::new();
        expected.insert("key".to_string(), Value::String("value".to_string()));
        assert_eq!(
            Value::from_str("{\"key\": \"value\"}"),
            Some(Value::Object(expected))
        );
    }

    #[test]
    fn test_parse_nested() {
        let mut inner = Object::new();
        inner.insert("inner_key".to_string(), Value::Number(42.0));

        let mut outer = Object::new();
        outer.insert("outer_key".to_string(), Value::Object(inner));

        assert_eq!(
            Value::from_str("{\"outer_key\": {\"inner_key\": 42}}"),
            Some(Value::Object(outer))
        );
    }

    #[test]
    fn test_invalid_json() {
        assert_eq!(Value::from_str("{key: value}"), None); // Missing quotes
        assert_eq!(Value::from_str("[1, 2,]"), None); // Trailing comma
        assert_eq!(Value::from_str("{\"key\":}"), None); // Incomplete object
        assert_eq!(Value::from_str("\"unterminated string"), None); // Unclosed string
    }
}
