use crate::{Element, Environment, EvaluationError};

pub(super) fn modulo(environment: &mut Environment) -> Result<(), EvaluationError> {
    if let Element::Integer(b) = environment.pop_value()? {
        if b == 0 {
            return Err(EvaluationError::DivisionByZero);
        }
        if let Element::Integer(a) = environment.pop_value()? {
            environment.push(Element::Integer(a % b))
        } else {
            Err(EvaluationError::InvalidStackElements)
        }
    } else {
        Err(EvaluationError::InvalidStackElements)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;
    use crate::Element;
    use super::*;

    #[test]
    fn returns_remainder_of_2_integers() {
        let mut env = Environment::new();
        assert_matches!(env.evaluate("4 3 %"), Ok(()));
        assert_matches!(env.pop(), Ok(Element::Integer(1)));
        assert_matches!(env.pop(), Err(EvaluationError::EmptyStack));
    }

    #[test]
    fn cannot_modulo_with_zero() {
        let mut env = Environment::new();
        assert_matches!(env.evaluate("4 0 %"), Err(EvaluationError::DivisionByZero));
    }
}