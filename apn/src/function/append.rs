use crate::{Element, Environment, EvaluationError};

pub(super) fn append(environment: &mut Environment) -> Result<(), EvaluationError> {
    if let Ok(b) = environment.pop() {
        if let Ok(Element::Array(a)) = environment.pop() {
            let mut c = a.clone();
            c.push(b);
            environment.push(Element::Array(c))
        } else {
            Err(EvaluationError::InvalidStackElements)
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
    fn appends_value_after_array() {
        let mut env = Environment::new();
        assert_matches!(env.evaluate("[1 2] 3 append"), Ok(()));
        if let Ok(Element::Array(array)) = env.pop() {
            assert_eq!(array.len(), 3);
            assert_matches!(array[0], Element::Integer(1));
            assert_matches!(array[1], Element::Integer(2));
            assert_matches!(array[2], Element::Integer(3));
        } else {
            assert!(false);
        }
        assert_matches!(env.pop(), Err(EvaluationError::EmptyStack));
    }
}