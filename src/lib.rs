mod parser;
mod scanner;

#[cfg(test)]
mod tests {
    use crate::scanner::{tokenize, Token};

    #[test]
    fn test_empty_object() {
        let input = "{}".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, vec![Token::LeftBrace, Token::RightBrace]);
    }

    #[test]
    fn test_empty_array() {
        let input = "[]".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, vec![Token::LeftBracket, Token::RightBracket]);
    }

    #[test]
    fn test_simple_string() {
        let input = "\"hello\"".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, vec![Token::String("hello".to_string())]);
    }

    #[test]
    fn test_boolean_true() {
        let input = "true".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, vec![Token::Boolean(true)]);
    }

    #[test]
    fn test_boolean_false() {
        let input = "false".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, vec![Token::Boolean(false)]);
    }

    #[test]
    fn test_null() {
        let input = "null".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, vec![Token::Null]);
    }

    #[test]
    fn test_number() {
        let input = "42.5".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, vec![Token::Number(42.5)]);
    }

    #[test]
    fn test_complex_json() {
        let input = "{\"key\": 123, \"flag\": true}".to_string();
        let tokens = tokenize(input);
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
