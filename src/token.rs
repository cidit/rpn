use crate::{errors::TokenParseError, operator::Operator};

#[derive(Debug)]
pub enum Token {
    Value(f32),
    Operator(Operator),
}

impl std::str::FromStr for Token {
    type Err = TokenParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = s.parse::<f32>() {
            Ok(Token::Value(value))
        } else if let Ok(op) = s.parse::<Operator>() {
            Ok(Token::Operator(op))
        } else {
            Err(TokenParseError::new(s.to_owned()))
        }
    }
}
