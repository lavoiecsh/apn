use crate::Environment;
use crate::environment::EvaluationError;

pub(super) fn pop(environment: &mut Environment) -> Result<(), EvaluationError> {
    environment.pop().map(|_| ())
}

#[cfg(test)]
mod tests {
    use crate::element::Element::Integer;
    use crate::Environment;
    use std::assert_matches::assert_matches;

    #[test]
    fn removes_an_item_from_stack() {
        let mut env = Environment::new();
        env.push(Integer(1)).unwrap();
        let result = env.evaluate("pop");
        assert_matches!(result, Ok(()));
        assert_eq!(env.stack_len(), 0);
    }
}