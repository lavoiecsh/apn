mod add;
mod subtract;

use crate::environment::EvaluationError;
use crate::function::add::add;
use crate::function::subtract::subtract;
use crate::Environment;

#[derive(Debug)]
pub(crate) struct Function {
    f: fn (&mut Environment) -> Result<(), EvaluationError>,
}

impl Function {
    pub(crate) fn execute(&self, environment: &mut Environment) -> Result<(), EvaluationError> {
        (self.f)(environment)
    }
}

#[derive(Debug, PartialEq)]
pub enum ExecutionError {}

impl TryFrom<&str> for Function {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "add" | "+" => Ok(Function { f: add }),
            "subtract" | "-" => Ok(Function { f: subtract }),
            _ => Err(()),
        }
    }
}