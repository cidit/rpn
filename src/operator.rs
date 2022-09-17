use crate::errors::EvaluationError;

#[derive(Debug)]
pub enum Operator {
    Additon,
    Subtraction,
    Multiplication,
    Division,
    // Exp,
}

impl Operator {
    /**
     * how many operands does this operator need?
     */
    pub fn num_operands(&self) -> usize {
        match self {
            Self::Additon => 2,
            Self::Subtraction => 2,
            Self::Multiplication => 2,
            Self::Division => 2,
        }
    }

    //takes the operands in the order of their operations
    pub fn apply(&self, operands: &[f32]) -> Result<f32, EvaluationError> {
        if self.num_operands() as usize != operands.len() {
            return Err(EvaluationError::NotEnoughData {
                actual: self.num_operands(),
                expected: operands.len(),
            });
        }

        let result = match self {
            Self::Additon => operands[0] + operands[1],
            Self::Subtraction => operands[0] - operands[1],
            Self::Multiplication => operands[0] * operands[1],
            Self::Division => {
                if operands[1] == 0f32 {
                    return Err(EvaluationError::DivisionByZero);
                } else {
                    operands[0] / operands[1]
                }
            }
        };

        return Ok(result);
    }
}

impl std::str::FromStr for Operator {
    // we dont care about error details, we just care to know if it could parse it or not
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Additon),
            "-" => Ok(Self::Subtraction),
            "/" => Ok(Self::Division),
            "*" => Ok(Self::Multiplication),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_addition() {
        let operands = [1.0, 2.0];
        let result = Operator::Additon.apply(&operands).unwrap();
        assert_eq!(3f32, result)
    }

    
}