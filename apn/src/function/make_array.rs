use crate::{Element, Environment, EvaluationError};

pub(super) fn make_array(environment: &mut Environment) -> Result<(), EvaluationError> {
    if let Ok(Element::Integer(n)) = environment.pop() {
        if n <= 0 {
            Err(EvaluationError::InvalidStackElements)
        } else {
            let mut array = Vec::new();
            for _ in 0..n {
                array.push(environment.pop()?);
            }
            array.reverse();
            environment.push(Element::Array(array))
        }
    } else {
        Err(EvaluationError::InvalidStackElements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;

    #[test]
    fn makes_an_array_from_multiple_elements() {
        let mut env = Environment::new();
        assert_matches!(env.evaluate("10 20 30 40 50 3 make_array"), Ok(()));
        if let Ok(Element::Array(array)) = env.pop() {
            assert_eq!(array.len(), 3);
            assert_matches!(array[0], Element::Integer(30));
            assert_matches!(array[1], Element::Integer(40));
            assert_matches!(array[2], Element::Integer(50));
        } else {
            assert!(false);
        }
        assert_matches!(env.pop(), Ok(Element::Integer(20)));
        assert_matches!(env.pop(), Ok(Element::Integer(10)));
        assert_matches!(env.pop(), Err(EvaluationError::EmptyStack));
    }

    #[test]
    fn errors_on_negative_integer() {
        let mut env = Environment::new();
        env.evaluate("10 20").unwrap();
        assert_matches!(env.evaluate("-1 make_array"), Err(EvaluationError::InvalidStackElements));
        assert_matches!(env.evaluate("0 make_array"), Err(EvaluationError::InvalidStackElements));
    }
}