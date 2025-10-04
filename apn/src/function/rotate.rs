use crate::{Environment, EvaluationError};

pub(super) fn rotate(environment: &mut Environment) -> Result<(), EvaluationError> {
    let b = environment.pop()?;
    let a = environment.pop()?;
    environment.push(b)?;
    environment.push(a)
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;
    use crate::Element;
    use super::*;

    #[test]
    fn rotates_top_two_elements_of_stack() {
        let mut env = Environment::new();
        assert_matches!(env.evaluate("1 2 rotate"), Ok(()));
        assert_matches!(env.pop(), Ok(Element::Integer(1)));
        assert_matches!(env.pop(), Ok(Element::Integer(2)));
        assert_matches!(env.pop(), Err(EvaluationError::EmptyStack));
    }

    #[test]
    fn rotates_top_two_elements_without_evaluating_them() {
        let mut env = Environment::new();
        assert_matches!(env.evaluate("$a [2] rotate"), Ok(()));
        assert_matches!(env.pop(), Ok(Element::Variable(_)));
        assert_matches!(env.pop(), Ok(Element::Array(_)));
        assert_matches!(env.pop(), Err(EvaluationError::EmptyStack));
    }
}