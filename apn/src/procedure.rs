use crate::element::Element;
use crate::function::Function;
use crate::parser::Token;
use crate::{Environment, EvaluationError};

#[derive(Debug)]
pub(super) struct Procedure(Vec<Token>);

impl Procedure {
    pub(super) fn execute(&self, environment: &mut Environment) -> Result<(), EvaluationError> {
        self.0.iter()
            .map(|t| environment.evaluate_token(t))
            .collect()
    }
}

impl TryFrom<&str> for Procedure {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "double" => Ok(Procedure(vec![
                Token::Element(Element::try_from("2")?),
                Token::Function(Function::try_from("*")?),
            ])),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::element::Element::Integer;
    use crate::Environment;
    use std::assert_matches::assert_matches;

    #[test]
    fn executes_defined_procedures() {
        let mut env = Environment::new();
        let result = env.evaluate("3 double");
        assert_matches!(result, Ok(()));
        assert_eq!(env.stack_len(), 1);
        assert_matches!(env.pop_value(), Ok(Integer(6)));
    }
}
