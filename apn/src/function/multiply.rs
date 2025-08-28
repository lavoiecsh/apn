use crate::{Environment, EvaluationError};
use crate::element::Element::{Float, Integer};

pub(super) fn multiply(environment: &mut Environment) -> Result<(), EvaluationError> {
    let b = environment.pop()?;
    let a = environment.pop()?;
    environment.push(match (a, b) {
        (Integer(a), Integer(b)) => Integer(a * b),
        (Integer(a), Float(b)) => Float(a as f64 * b),
        (Float(a), Integer(b)) => Float(a * b as f64),
        (Float(a), Float(b)) => Float(a * b),
        _ => return Err(EvaluationError::FunctionNotApplicable),
    })
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;
    use super::*;

    #[test]
    fn multiplies_last_2_numbers() {
        let mut env = Environment::new();
        env.push(Integer(2)).unwrap();
        env.push(Float(3.5)).unwrap();
        let result = env.evaluate("*");
        assert_matches!(result, Ok(()));
        assert_eq!(env.stack_len(), 1);
        assert_matches!(env.pop(), Ok(Float(7.)));
    }
}