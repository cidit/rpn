use thiserror::Error;

#[derive(Debug, Error)]
pub enum EvaluationError {
    #[error(transparent)]
    TokenParseError(#[from] TokenParseError),
    // FIXME: not explicit enough. what if actual is bigger than expected?
    #[error("not enough data to compute all operations'. expected {expected} operators, found {actual}.")]
    NotEnoughData { expected: usize, actual: usize },
    #[error("not enough operators to compute all the values. remaining stack: {stack:?}.")]
    TooManyLeft { stack: Vec<f32> },
    #[error("there were not enough values in the stack to finish evaluation.")]
    NoValueInStack,
    #[error("divided by zero")]
    DivisionByZero,
}

#[derive(Debug, Error)]
#[error("impossible to parse token: {got}")]
pub struct TokenParseError {
    got: String,
}

impl TokenParseError {
    pub fn new(got: String) -> Self {
        Self { got }
    }
}
