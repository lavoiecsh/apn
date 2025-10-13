use crate::{Environment, EvaluationError};

pub(super) fn copy(environment: &mut Environment) -> Result<(), EvaluationError> {
    let element = environment.pop()?;
    environment.push(element.clone())?;
    environment.push(element)
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;
    use crate::Element;
    use super::*;
    
    #[test]
    fn copies_top_element() {
        let mut env = Environment::new();
        assert_matches!(env.evaluate("1 copy"), Ok(()));
        assert_matches!(env.pop(), Ok(Element::Integer(1)));
        assert_matches!(env.pop(), Ok(Element::Integer(1)));
        assert_matches!(env.pop(), Err(EvaluationError::EmptyStack));
    }
    
    #[test]
    fn copies_without_evaluating() {
        let mut env = Environment::new();
        assert_matches!(env.evaluate("$a copy"), Ok(()));
        assert_matches!(env.pop(), Ok(Element::Variable(name)) if name == "a");
        assert_matches!(env.pop(), Ok(Element::Variable(name)) if name == "a");
        assert_matches!(env.pop(), Err(EvaluationError::EmptyStack));
    }
}