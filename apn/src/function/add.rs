use crate::element::Element::{Float, Integer};
use crate::environment::EvaluationError;
use crate::Environment;

pub(super) fn add(environment: &mut Environment) -> Result<(), EvaluationError> {
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
    use super::*;
    use std::assert_matches::assert_matches;

    #[test]
    fn adds_2_integers_into_integer() {
        let mut env = Environment::new();
        env.push(Integer(1)).unwrap();
        env.push(Integer(2)).unwrap();
        let result = env.evaluate("add");
        assert_matches!(result, Ok(()));
        assert_eq!(env.stack_len(), 1);
        assert_eq!(env.pop(), Ok(Integer(3)));
    }

    #[test]
    fn adds_2_floats_into_float() {
        let mut env = Environment::new();
        env.push(Float(1.1)).unwrap();
        env.push(Float(2.4)).unwrap();
        let result = add(&mut env);
        assert_matches!(result, Ok(()));
        assert_eq!(env.stack_len(), 1);
        assert_eq!(env.pop(), Ok(Float(3.5)));
    }
}