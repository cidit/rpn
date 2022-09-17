mod errors;
mod evaluable;
mod operator;
mod token;
mod token_stack;

#[cfg(test)]
mod tests {

    use crate::evaluable::Evaluable;

    #[test]
    fn test_evaluate() {
        assert_eq!(1.0, "-4 5 +".evaluate().unwrap());
        assert_eq!(2.5, "5 2 /".evaluate().unwrap());
        assert_eq!(2.0, "5 2.5 /".evaluate().unwrap());
        assert_eq!(14.0, "5 1 2 + 4 * 3 - +".evaluate().unwrap());
        assert_eq!(2.0, "4 2 5 * + 1 3 2 * + /".evaluate().unwrap());
    }
}
