use crate::{Environment, EvaluationError};
use crate::element::Element::{Float, Integer};

pub(super) fn divide(environment: &mut Environment) -> Result<(), EvaluationError> {
    let b = environment.pop_value()?;
    if match b {
        Integer(b) => b == 0,
        Float(b) => b == 0.0,
        _ => return Err(EvaluationError::FunctionNotApplicable),
    } {
        return Err(EvaluationError::DivisionByZero);
    }
    let a = environment.pop_value()?;
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
        assert_matches!(env.evaluate("3 2 /"), Ok(()));
        assert_eq!(env.stack_len(), 1);
        assert_matches!(env.pop_value(), Ok(Float(1.5)));
    }

    #[test]
    fn errs_if_dividing_by_zero() {
        let mut env = Environment::new();
        assert_matches!(env.evaluate("2 0"), Ok(()));
        assert_matches!(env.evaluate("/"), Err(EvaluationError::DivisionByZero));
        assert_eq!(env.stack_len(), 2);
    }
}