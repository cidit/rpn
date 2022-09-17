use crate::{errors::EvaluationError, token_stack::TokenStack};

pub trait Evaluable {
    fn evaluate(self) -> Result<f32, EvaluationError>;
}

impl Evaluable for &str {
    fn evaluate(self) -> Result<f32, EvaluationError> {
        self.parse::<TokenStack>()?.evaluate()
    }
}
