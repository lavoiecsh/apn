use crate::element::Element::{Float, Integer};
use crate::Environment;
use crate::environment::EvaluationError;

pub(crate) fn subtract(environment: &mut Environment) -> Result<(), EvaluationError> {
    let b = environment.pop()?;
    let a = environment.pop()?;
    environment.push(match (a, b) {
        (Integer(a), Integer(b)) => Integer(a - b),
        (Float(a), Float(b)) => Float(a - b),
        (Float(a), Integer(b)) => Float(a - b as f64),
        (Integer(a), Float(b)) => Float(a as f64 - b),
    })
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;
    use super::*;

    #[test]
    fn subtracts_2_integers_into_integer() {
        let mut environment = Environment::new();
        environment.push(Integer(2)).unwrap();
        environment.push(Integer(3)).unwrap();
        let result = environment.evaluate("-");
        assert_matches!(result, Ok(()));
        assert_eq!(environment.stack_len(), 1);
        assert_matches!(environment.pop(), Ok(Integer(-1)));
    }
}