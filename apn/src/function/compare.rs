use crate::element::Element::{Boolean, Float, Integer};
use crate::{Environment, EvaluationError};

pub(super) fn less(environment: &mut Environment) -> Result<(), EvaluationError> {
    let b = environment.pop_value()?;
    let a = environment.pop_value()?;
    environment.push(match (a, b) {
        (Integer(a), Integer(b)) => Boolean(a < b),
        (Integer(a), Float(b)) => Boolean((a as f64) < b),
        (Float(a), Integer(b)) => Boolean(a < b as f64),
        (Float(a), Float(b)) => Boolean(a < b),
        _ => return Err(EvaluationError::FunctionNotApplicable),
    })
}

pub(super) fn less_equal(environment: &mut Environment) -> Result<(), EvaluationError> {
    let b = environment.pop_value()?;
    let a = environment.pop_value()?;
    environment.push(match (a, b) {
        (Integer(a), Integer(b)) => Boolean(a <= b),
        (Integer(a), Float(b)) => Boolean((a as f64) <= b),
        (Float(a), Integer(b)) => Boolean(a <= b as f64),
        (Float(a), Float(b)) => Boolean(a <= b),
        _ => return Err(EvaluationError::FunctionNotApplicable),
    })
}

pub(super) fn equal(environment: &mut Environment) -> Result<(), EvaluationError> {
    let b = environment.pop_value()?;
    let a = environment.pop_value()?;
    environment.push(match (a, b) {
        (Integer(a), Integer(b)) => Boolean(a == b),
        (Integer(a), Float(b)) => Boolean((a as f64) == b),
        (Float(a), Integer(b)) => Boolean(a == b as f64),
        (Float(a), Float(b)) => Boolean(a == b),
        (Boolean(a), Boolean(b)) => Boolean(a == b),
        _ => return Err(EvaluationError::FunctionNotApplicable),
    })
}

pub(super) fn greater(environment: &mut Environment) -> Result<(), EvaluationError> {
    let b = environment.pop_value()?;
    let a = environment.pop_value()?;
    environment.push(match (a, b) {
        (Integer(a), Integer(b)) => Boolean(a > b),
        (Integer(a), Float(b)) => Boolean((a as f64) > b),
        (Float(a), Integer(b)) => Boolean(a > b as f64),
        (Float(a), Float(b)) => Boolean(a > b),
        _ => return Err(EvaluationError::FunctionNotApplicable),
    })
}

pub(super) fn greater_equal(environment: &mut Environment) -> Result<(), EvaluationError> {
    let b = environment.pop_value()?;
    let a = environment.pop_value()?;
    environment.push(match (a, b) {
        (Integer(a), Integer(b)) => Boolean(a >= b),
        (Integer(a), Float(b)) => Boolean((a as f64) >= b),
        (Float(a), Integer(b)) => Boolean(a >= b as f64),
        (Float(a), Float(b)) => Boolean(a >= b),
        _ => return Err(EvaluationError::FunctionNotApplicable),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;

    #[test]
    fn compares_numbers() {
        let mut env = Environment::new();
        let result = env.evaluate("1 2 < 3 2 <= 4 4 == 5 6 > 7 6 >=");
        assert_matches!(result, Ok(()));
        let results: Vec<bool> = env
            .stack()
            .filter_map(|e| if let Boolean(b) = e { Some(b) } else { None })
            .cloned()
            .collect();
        assert_eq!(results, vec![true, false, true, false, true]);
    }
}
