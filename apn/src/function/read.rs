use crate::{Environment, EvaluationError};
use std::io::Error;
use std::fs;

pub(super) fn read(environment: &mut Environment) -> Result<(), EvaluationError> {
    let filename = environment.pop()?.as_string()?;
    let contents = fs::read_to_string(filename)?;
    environment.evaluate(contents)
}

impl From<Error> for EvaluationError {
    fn from(value: Error) -> Self {
        EvaluationError::IO(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Element;
    use std::assert_matches::assert_matches;

    #[test]
    fn reads_file_and_evaluates_it() {
        let mut env = Environment::new();
        assert_matches!(env.evaluate("\"../programs/inc.apn\" read"), Ok(()));
        assert_matches!(env.evaluate("2 $inc ."), Ok(()));
        assert_matches!(env.pop(), Ok(Element::Integer(3)));
        assert_matches!(env.pop(), Err(EvaluationError::EmptyStack));
    }
}