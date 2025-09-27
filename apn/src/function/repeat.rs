use crate::{Environment, EvaluationError};
use crate::element::Element;

pub(super) fn repeat(environment: &mut Environment) -> Result<(), EvaluationError> {
    if let Element::Integer(count) = environment.pop()? {
        let element = environment.pop()?;
        for _ in 0..count {
            environment.push(element.clone())?;
        }
        Ok(())
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
    fn repeats_previous_element_n_times() {
        let mut env = Environment::new();
        let result = env.evaluate("1 5 repeat");
        assert_matches!(result, Ok(()));
        assert_eq!(env.stack_len(), 5);
        for _ in 0..5 {
            assert_matches!(env.pop(), Ok(Element::Integer(1)));
        }
    }
}