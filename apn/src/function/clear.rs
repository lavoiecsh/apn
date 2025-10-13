use crate::{Environment, EvaluationError};

pub(super) fn clear(environment: &mut Environment) -> Result<(), EvaluationError> {
    while environment.stack_len() != 0 {
        environment.pop()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;
    use super::*;
    
    #[test]
    fn clears_stack() {
        let mut env = Environment::new();
        assert_matches!(env.evaluate("1 2"), Ok(()));
        assert_matches!(env.evaluate("clear"), Ok(()));
        assert_eq!(env.stack_len(), 0);
    }
}