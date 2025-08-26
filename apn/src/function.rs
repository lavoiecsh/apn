use crate::environment::{Environment, EvaluationError};

pub(crate) fn add(environment: &mut Environment) -> Result<(), EvaluationError> {
    let b = environment.pop()?;
    let a = environment.pop()?;
    let c = a.add(b)?;
    environment.push(c)
}

pub(crate) fn subtract(environment: &mut Environment) -> Result<(), EvaluationError> {
    let b = environment.pop()?;
    let a = environment.pop()?;
    let c = a.subtract(b)?;
    environment.push(c)
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
        assert_matches!(result, Ok(_));
        assert_eq!(env.stack_len(), 1);
        assert_matches!(env.pop(), Ok(Element::Integer(3)));
    }
}