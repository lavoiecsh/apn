use crate::environment::{Environment, EvaluationError};

pub(crate) fn add(environment: &mut Environment) -> Result<(), EvaluationError> {
    let b = environment.pop()?;
    let a = environment.pop()?;
    environment.push(a.add(b)?)
}

pub(crate) fn subtract(environment: &mut Environment) -> Result<(), EvaluationError> {
    let b = environment.pop()?;
    let a = environment.pop()?;
    environment.push(a.subtract(b)?)
}

pub(crate) fn multiply(environment: &mut Environment) -> Result<(), EvaluationError> {
    let b = environment.pop()?;
    let a = environment.pop()?;
    environment.push(a.multiply(b)?)
}

pub(crate) fn divide(environment: &mut Environment) -> Result<(), EvaluationError> {
    let b = environment.pop()?;
    let a = environment.pop()?;
    environment.push(a.divide(b)?)
}

pub(crate) fn modulo(environment: &mut Environment) -> Result<(), EvaluationError> {
    let b = environment.pop()?;
    let a = environment.pop()?;
    environment.push(a.modulo(b)?)
}

pub(crate) fn floor(environment: &mut Environment) -> Result<(), EvaluationError> {
    let a = environment.pop()?;
    environment.push(a.floor()?)
}

pub(crate) fn ceiling(environment: &mut Environment) -> Result<(), EvaluationError> {
    let a = environment.pop()?;
    environment.push(a.ceiling()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;
    use crate::element::Element;

    #[test]
    fn adds_last_2_elements_and_pushes_result() {
        let mut env = Environment::new();
        env.push(Element::Integer(1)).unwrap();
        env.push(Element::Integer(2)).unwrap();
        let result = add(&mut env);
        assert_matches!(result, Ok(()));
        assert_eq!(env.stack_len(), 1);
        assert_matches!(env.pop(), Ok(Element::Integer(3)));
    }

    #[test]
    fn subtracts_last_2_elements_and_pushes_result() {
        let mut env = Environment::new();
        env.push(Element::Integer(3)).unwrap();
        env.push(Element::Integer(1)).unwrap();
        let result = subtract(&mut env);
        assert_matches!(result, Ok(()));
        assert_eq!(env.stack_len(), 1);
        assert_matches!(env.pop(), Ok(Element::Integer(2)));
    }

    #[test]
    fn multiplies_last_2_elements_and_pushes_result() {
        let mut env = Environment::new();
        env.push(Element::Integer(3)).unwrap();
        env.push(Element::Integer(4)).unwrap();
        let result = multiply(&mut env);
        assert_matches!(result, Ok(()));
        assert_eq!(env.stack_len(), 1);
        assert_matches!(env.pop(), Ok(Element::Integer(12)));
    }
}