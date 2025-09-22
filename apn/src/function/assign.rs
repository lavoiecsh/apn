use crate::element::Element;
use crate::{Environment, EvaluationError};

pub(super) fn assign(environment: &mut Environment) -> Result<(), EvaluationError> {
    let variable = environment.pop()?;
    if let Element::Variable(name) = variable {
        let value = environment.pop()?;
        environment.assign(name, value)
    } else {
        Err(EvaluationError::InvalidStackElements)
    }
}

#[cfg(test)]
mod tests {
    use crate::element::Element::{Float, Integer};
    use crate::Environment;
    use std::assert_matches::assert_matches;

    #[test]
    fn assigns_variable_to_value() {
        let mut env = Environment::new();
        let result = env.evaluate("1 $test assign");
        assert_matches!(result, Ok(()));
        assert_eq!(env.stack_len(), 0);
        env.evaluate("$test").unwrap();
        assert_matches!(env.pop_value(), Ok(Integer(1)));
    }

    #[test]
    fn overrides_variables_value() {
        let mut env = Environment::new();
        let result = env.evaluate("1 $test = 3. $test =");
        assert_matches!(result, Ok(()));
        assert_eq!(env.stack_len(), 0);
        env.evaluate("$test").unwrap();
        assert_matches!(env.pop_value(), Ok(Float(3.)));
    }

    #[test]
    fn assigns_as_reference_to_other_variable() {
        let mut env = Environment::new();
        let result = env.evaluate("1 $test1 = $test1 $test2 =");
        assert_matches!(result, Ok(()));
        env.evaluate("$test2").unwrap();
        assert_matches!(env.pop_value(), Ok(Integer(1)));
        env.evaluate("2 $test1 = $test2").unwrap();
        assert_matches!(env.pop_value(), Ok(Integer(2)));
    }
}