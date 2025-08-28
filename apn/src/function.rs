mod add;
mod pop;
mod subtract;
mod multiply;
mod divide;

use crate::{Environment, EvaluationError};

use crate::function::add::add;
use crate::function::divide::divide;
use crate::function::multiply::multiply;
use crate::function::pop::pop;
use crate::function::subtract::subtract;

#[derive(Debug)]
pub(super) struct Function(fn (&mut Environment) -> Result<(), EvaluationError>);

impl Function {
    pub(super) fn execute(&self, environment: &mut Environment) -> Result<(), EvaluationError> {
        self.0(environment)
    }
}

#[derive(Debug, PartialEq)]
pub enum ExecutionError {}

impl TryFrom<&str> for Function {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "add" | "+" => Ok(Function(add)),
            "subtract" | "-" => Ok(Function(subtract)),
            "multiply" | "*" => Ok(Function(multiply)),
            "divide" | "/" => Ok(Function(divide)),
            "pop" => Ok(Function(pop)),
            _ => Err(()),
        }
    }
}