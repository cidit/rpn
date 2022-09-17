use crate::{
    errors::{EvaluationError, TokenParseError},
    token::Token,
};

pub struct TokenStack {
    tokens: Vec<Token>,
}

impl TokenStack {
    pub fn evaluate(self) -> Result<f32, EvaluationError> {
        let mut stack: Vec<f32> = Vec::new();

        for token in self.tokens {
            match token {
                Token::Value(value) => stack.push(value),
                Token::Operator(op) => {
                    if stack.len() < op.num_operands() {
                        return Err(EvaluationError::NotEnoughData {
                            expected: op.num_operands(),
                            actual: stack.len(),
                        });
                    }
                    let idx = stack.len() - op.num_operands();
                    let operands: Vec<_> = stack.drain(idx..).collect();
                    let result = op.apply(&operands)?;
                    stack.push(result);
                }
            }
        }

        if stack.len() > 1 {
            return Err(EvaluationError::TooManyLeft { stack });
        }
        match stack.last() {
            Some(&value) => Ok(value),
            None => Err(EvaluationError::NoValueInStack),
        }
    }
}

impl std::str::FromStr for TokenStack {
    type Err = TokenParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s
            .split(' ')
            .map(Token::from_str)
            .collect::<Result<_, _>>()?;
        Ok(Self { tokens })
    }
}
