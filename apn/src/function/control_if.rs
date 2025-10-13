use crate::{Environment, EvaluationError};
use crate::element::Element;

pub(super) fn control_if(environment: &mut Environment) -> Result<(), EvaluationError> {
    let result_false = environment.pop()?;
    let result_true = environment.pop()?;
    match environment.pop_value()? {
        Element::Boolean(true) => environment.push(result_true),
        Element::Boolean(false) => environment.push(result_false),
        _ => Err(EvaluationError::InvalidStackElements),
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;
    use crate::element::Element;
    use super::*;

    #[test]
    fn keeps_first_element_if_true() {
        let mut env = Environment::new();
        let result = env.evaluate("true 1 2 if");
        assert_matches!(result, Ok(()));
        assert_eq!(env.stack_len(), 1);
        assert_matches!(env.pop(), Ok(Element::Integer(1)));
    }

    #[test]
    fn keeps_second_element_if_false() {
        let mut env = Environment::new();
        let result = env.evaluate("false 1 2 if");
        assert_matches!(result, Ok(()));
        assert_eq!(env.stack_len(), 1);
        assert_matches!(env.pop(), Ok(Element::Integer(2)));
    }
}