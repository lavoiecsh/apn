use crate::element::Element;
use crate::{Environment, EvaluationError};

pub(super) fn eval(environment: &mut Environment) -> Result<(), EvaluationError> {
    let element = environment.pop_value()?;
    match element {
        Element::Array(elements) => {
            for e in elements {
                environment.push(e)?;
            }
            Ok(())
        }
        _ => environment.push(element),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;

    #[test]
    fn evaluates_simple_element_as_itself() {
        let mut env = Environment::new();
        assert_matches!(env.evaluate("1 eval"), Ok(()));
        assert_matches!(env.pop(), Ok(Element::Integer(1)));
        assert_matches!(env.pop(), Err(EvaluationError::EmptyStack));
    }

    #[test]
    fn evaluates_array_as_single_elements() {
        let mut env = Environment::new();
        assert_matches!(env.evaluate("[1 2 3] eval"), Ok(()));
        assert_matches!(env.pop(), Ok(Element::Integer(3)));
        assert_matches!(env.pop(), Ok(Element::Integer(2)));
        assert_matches!(env.pop(), Ok(Element::Integer(1)));
        assert_matches!(env.pop(), Err(EvaluationError::EmptyStack));
    }

    #[test]
    fn evaluates_variable_as_value() {
        let mut env = Environment::new();
        assert_matches!(env.evaluate("1 $x = $x eval"), Ok(()));
        assert_matches!(env.pop(), Ok(Element::Integer(1)));
        assert_matches!(env.pop(), Err(EvaluationError::EmptyStack));
    }

    #[test]
    fn evaluates_array_with_function() {
        let mut env = Environment::new();
        assert_matches!(env.evaluate("[1 2 +] eval"), Ok(()));
        assert_matches!(env.pop(), Ok(Element::Integer(3)));
        assert_matches!(env.pop(), Err(EvaluationError::EmptyStack));
    }

    #[test]
    fn evaluates_array_with_function_inside_variable() {
        let mut env = Environment::new();
        assert_matches!(env.evaluate("[1 2 +] $x = $x eval"), Ok(()));
        assert_matches!(env.pop(), Ok(Element::Integer(3)));
        assert_matches!(env.pop(), Err(EvaluationError::EmptyStack));
    }
}