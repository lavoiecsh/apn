use crate::{Environment, EvaluationError};
use crate::element::Element;

pub(super) fn append(environment: &mut Environment) -> Result<(), EvaluationError> {
    if let Ok(Element::Array(b)) = environment.pop() {
        if let Ok(Element::Array(a)) = environment.pop() {
            let c = a.into_iter()
                .chain(b.into_iter())
                .collect();
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
    use std::assert_matches::assert_matches;
    use crate::element::Element;
    use super::*;

    #[test]
    fn appends_two_arrays() {
        let mut env = Environment::new();
        assert_matches!(env.evaluate("[1] [2] append"), Ok(()));
        if let Ok(Element::Array(array)) = env.pop() {
            assert_matches!(array[0], Element::Integer(1));
            assert_matches!(array[1], Element::Integer(2));
        } else {
            assert!(false);
        }
        assert_matches!(env.pop(), Err(EvaluationError::EmptyStack));
    }
}