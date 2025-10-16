use crate::{Element, Environment, EvaluationError};
use crate::function::eval::eval;
use crate::function::Function;
use crate::function::make_array::make_array;

pub(super) fn map(environment: &mut Environment) -> Result<(), EvaluationError> {
    if let Element::Procedure(proc) = environment.pop()? {
        if let Element::Array(array) = environment.pop()? {
            let len = array.len();
            for e in array {
                environment.push(e)?;
                environment.push(Element::Procedure(proc.clone()))?;
                environment.push(Element::Function(Function("eval", eval)))?;
            }
            environment.push(Element::Integer(len as i64))?;
            environment.push(Element::Function(Function("make_array", make_array)))
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
    fn applies_procedure_to_each_item_in_array() {
        let mut env = Environment::new();
        assert_matches!(env.evaluate("[1 2 3 4 5] {2 *} map"), Ok(()));
        assert_matches!(env.pop(), Ok(Element::Array(array)) if
        matches!(array[0], Element::Integer(2)) &&
        matches!(array[1], Element::Integer(4)) &&
        matches!(array[2], Element::Integer(6)) &&
        matches!(array[3], Element::Integer(8)) &&
        matches!(array[4], Element::Integer(10)));
        assert_matches!(env.pop(), Err(EvaluationError::EmptyStack));
    }
}