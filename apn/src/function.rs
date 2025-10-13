mod add;
mod pop;
mod subtract;
mod multiply;
mod divide;
mod compare;
mod assign;
mod control_if;
mod repeat;
mod eval;
mod concatenate;
mod rotate;
mod append;
mod make_array;
mod read;
mod modulo;

use crate::{Environment, EvaluationError};

use crate::function::add::add;
use crate::function::append::append;
use crate::function::concatenate::concatenate;
use crate::function::assign::assign;
use crate::function::compare::{equal, greater, greater_equal, less, less_equal};
use crate::function::control_if::control_if;
use crate::function::divide::divide;
use crate::function::eval::eval;
use crate::function::make_array::make_array;
use crate::function::modulo::modulo;
use crate::function::multiply::multiply;
use crate::function::pop::pop;
use crate::function::read::read;
use crate::function::repeat::{repeat, repeat_eval};
use crate::function::subtract::subtract;
use crate::function::rotate::rotate;

#[derive(Debug, PartialEq, Clone)]
pub struct Function(&'static str, fn (&mut Environment) -> Result<(), EvaluationError>);

impl Function {
    pub(super) fn execute(&self, environment: &mut Environment) -> Result<(), EvaluationError> {
        self.1(environment)
    }

    pub(super) fn name(&self) -> &'static str {
        self.0
    }
}

#[derive(Debug, PartialEq)]
pub enum ExecutionError {}

impl TryFrom<&str> for Function {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            // math
            "add" | "+" => Ok(Function("+", add)),
            "subtract" | "-" => Ok(Function("-", subtract)),
            "multiply" | "*" => Ok(Function("*", multiply)),
            "divide" | "/" => Ok(Function("/", divide)),
            "modulo" | "%" => Ok(Function("%", modulo)),
            // comparison
            "less" | "<" => Ok(Function("<", less)),
            "less_equal" | "<=" => Ok(Function("<=", less_equal)),
            "equal" | "==" => Ok(Function("==", equal)),
            "greater" | ">" => Ok(Function(">", greater)),
            "greater_equal" | ">=" => Ok(Function(">=", greater_equal)),
            // stack manipulation
            "pop" => Ok(Function("pop", pop)),
            "rotate" => Ok(Function("rotate", rotate)),
            // control flow
            "assign" | "=" => Ok(Function("=", assign)),
            "if" => Ok(Function("if", control_if)),
            "eval" | "." => Ok(Function(".", eval)),
            "repeat" => Ok(Function("repeat", repeat)),
            "repeat_eval" | "repeat." => Ok(Function("repeat.", repeat_eval)),
            "read" => Ok(Function("read", read)),
            // array manipulation
            "concatenate" | "concat" | "++" => Ok(Function("++", concatenate)),
            "append" => Ok(Function("append", append)),
            "make_array" => Ok(Function("make_array", make_array)),
            // error
            _ => Err(()),
        }
    }
}