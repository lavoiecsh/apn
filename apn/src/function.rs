mod add;
mod pop;
mod subtract;
mod multiply;
mod divide;
mod compare;
mod assign;
mod control_if;
mod repeat;

use crate::{Environment, EvaluationError};

use crate::function::add::add;
use crate::function::assign::assign;
use crate::function::compare::{equal, greater, greater_equal, less, less_equal};
use crate::function::control_if::control_if;
use crate::function::divide::divide;
use crate::function::multiply::multiply;
use crate::function::pop::pop;
use crate::function::repeat::repeat;
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
            "less" | "<" => Ok(Function(less)),
            "less_equal" | "<=" => Ok(Function(less_equal)),
            "equal" | "==" => Ok(Function(equal)),
            "greater" | ">" => Ok(Function(greater)),
            "greater_equal" | ">=" => Ok(Function(greater_equal)),
            "pop" => Ok(Function(pop)),
            "assign" | "=" => Ok(Function(assign)),
            "if" => Ok(Function(control_if)),
            "repeat" => Ok(Function(repeat)),
            _ => Err(()),
        }
    }
}