use crate::scanner::Token;

pub type Object = HashMap<String, Value>;

pub enum Value {
    String(String),
    Array(Vec<Value>),
    Object(Object),
    Boolean(bool),
    Number(f64),
    Null,
}

pub fn parse(tokens: &[Token]) -> Option<Object> {
    let mut iter = tokens.iter();
    let mut result = Object::default();

    while let Some(t) = iter.next() {
        match t {
            Token::LeftBrace => todo!(),
            Token::Colon => todo!(),
            Token::String(_) => todo!(),
            Token::Number(_) => todo!(),
            Token::Boolean(_) => todo!(),
            Token::Null => todo!(),
            Token::Comma => todo!(),
            Token::LeftBracket => todo!(),
            Token::RightBracket => todo!(),
        }
    }
}

fn parse_key_value_pair(iter: &mut impl Iterator<Item = Token>) -> (String, Value) {
    if let Some(key) = iter.next() {
        match key {
            Token::String => (key, parse_value(iter)),
            _ => panic!(),
        }
    }
}

fn parse_value(iter: &mut impl Iterator<Item = Token>) -> Value {
    iter.next();

    if let Some(t) = iter.next() {
        match t {
            Token::Number(n) => Value::Number(n),
            Token::String(s) => Value::String(s),
            Token::Null => Value::Null,
            Token::RightBracket => parse_array(iter),
        }
    }
}

fn parse_array(iter: &mut impl Iterator<Item = Token>) -> Value {
    let array = Vec::new();
    while let Some(t) = iter.next() {
        if t == Token::RightBracket {
            break;
        }
        array.push(parse_value(iter));
        iter.next();
    }
    array
}
