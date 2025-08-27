use crate::element::Element::{Float, Integer};
use crate::Environment;
use crate::environment::EvaluationError;

pub(crate) fn add(environment: &mut Environment) -> Result<(), EvaluationError> {
    let b = environment.pop()?;
    let a = environment.pop()?;
    environment.push(match (a, b) {
        (Integer(a), Integer(b)) => Integer(a + b),
        (Float(a), Float(b)) => Float(a + b),
        (Integer(a), Float(b)) => Float(a as f64 + b),
        (Float(a), Integer(b)) => Float(a + b as f64),
    })
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;
    use super::*;

    #[test]
    fn adds_2_integers_into_integer() {
        let mut environment = Environment::new();
        environment.push(Integer(1)).unwrap();
        environment.push(Integer(2)).unwrap();
        let result = add(&mut environment);
        assert_matches!(result, Ok(()));
        assert_eq!(environment.stack_len(), 1);
        assert_eq!(environment.pop(), Ok(Integer(3)));
    }

    #[test]
    fn adds_2_floats_into_float() {
        let mut environment = Environment::new();
        environment.push(Float(1.1)).unwrap();
        environment.push(Float(2.4)).unwrap();
        let result = add(&mut environment);
        assert_matches!(result, Ok(()));
        assert_eq!(environment.stack_len(), 1);
        assert_eq!(environment.pop(), Ok(Float(3.5)));
    }
}