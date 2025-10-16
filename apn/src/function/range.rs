use crate::{Element, Environment, EvaluationError};

pub(super) fn range(environment: &mut Environment) -> Result<(), EvaluationError> {
    if let Element::Integer(b) = environment.pop_value()? {
        if let Element::Integer(a) = environment.pop_value()? {
            environment.push(Element::Array((a..=b).map(Element::Integer).collect()))
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
    use crate::Element;
    use std::assert_matches::assert_matches;

    #[test]
    fn creates_array_with_integers_between_range() {
        let mut env = Environment::new();
        assert_matches!(env.evaluate("1 5 range"), Ok(()));
        assert_matches!(env.pop(), Ok(Element::Array(array)) if
        matches!(array[0], Element::Integer(1)) &&
        matches!(array[1], Element::Integer(2)) &&
        matches!(array[2], Element::Integer(3)) &&
        matches!(array[3], Element::Integer(4)) &&
        matches!(array[4], Element::Integer(5)));
        assert_matches!(env.pop(), Err(EvaluationError::EmptyStack));
    }
}
