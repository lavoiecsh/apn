use crate::{Environment, EvaluationError};
use crate::element::Element::{Float, Integer};

pub(super) fn divide(environment: &mut Environment) -> Result<(), EvaluationError> {
    let b = environment.pop()?;
    if match b {
        Integer(b) => b == 0,
        Float(b) => b == 0.0,
        _ => return Err(EvaluationError::FunctionNotApplicable),
    } {
        return Err(EvaluationError::DivisionByZero);
    }
    let a = environment.pop()?;
    environment.push(match (a, b) {
        (Integer(a), Integer(b)) => Float(a as f64 / b as f64),
        (Float(a), Float(b)) => Float(a / b),
        (Integer(a), Float(b)) => Float(a as f64 / b),
        (Float(a), Integer(b)) => Float(a / b as f64),
        _ => return Err(EvaluationError::FunctionNotApplicable),
    })
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;
    use super::*;

    #[test]
    fn always_divides_as_float() {
        let mut env = Environment::new();
        env.push(Integer(3)).unwrap();
        env.push(Integer(2)).unwrap();
        let result = env.evaluate("/");
        assert_matches!(result, Ok(()));
        assert_eq!(env.stack_len(), 1);
        assert_matches!(env.pop(), Ok(Float(1.5)));
    }

    #[test]
    fn errs_if_dividing_by_zero() {
        let mut env = Environment::new();
        env.push(Integer(2)).unwrap();
        env.push(Integer(0)).unwrap();
        let result = env.evaluate("/");
        assert_matches!(result, Err(EvaluationError::DivisionByZero));
        assert_eq!(env.stack_len(), 2);
    }
}