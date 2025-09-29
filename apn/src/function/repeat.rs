use crate::{Environment, EvaluationError};
use crate::element::Element;
use crate::function::eval::eval;

pub(super) fn repeat(environment: &mut Environment) -> Result<(), EvaluationError> {
    if let Element::Integer(count) = environment.pop_value()? {
        let element = environment.pop()?;
        for _ in 0..count {
            environment.push(element.clone())?;
        }
        Ok(())
    } else {
        Err(EvaluationError::InvalidStackElements)
    }
}

pub(super) fn repeat_eval(environment: &mut Environment) -> Result<(), EvaluationError> {
    if let Element::Integer(count) = environment.pop_value()? {
        let element = environment.pop()?;
        for _ in 0..count {
            environment.push(element.clone())?;
            eval(environment)?;
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

    #[test]
    fn repeats_and_evaluates_n_times() {
        let mut env = Environment::new();
        assert_matches!(env.evaluate("1 2 3 4 5 [+] 4 repeat_eval"), Ok(()));
        assert_matches!(env.pop(), Ok(Element::Integer(15)));
        assert_matches!(env.pop(), Err(EvaluationError::EmptyStack));
    }
}