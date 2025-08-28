use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::element::{Element, ElementError};
use crate::parser::{parse, ParserError, Token};

pub struct Environment {
    stack: Vec<Element>,
    evaluation_history: Vec<EvaluationOperation>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            evaluation_history: Vec::new(),
        }
    }

    pub fn evaluate(&mut self, input: impl Into<String>) -> Result<(), EvaluationError> {
        self.evaluation_history.clear();
        let tokens = parse(input)?;
        if let Err(e) = self.evaluate_tokens(tokens) {
            for item in self.evaluation_history.iter().rev() {
                match item {
                    EvaluationOperation::Push => { self.stack.pop(); }
                    EvaluationOperation::Pop(element) => { self.stack.push(element.clone()); }
                }
            }
            Err(e)
        } else {
            Ok(())
        }
    }

    fn evaluate_tokens(&mut self, tokens: Vec<Token>) -> Result<(), EvaluationError> {
        for token in tokens {
            match token {
                Token::Element(e) => self.stack.push(e),
                Token::Function(f) => f.execute(self)?,
            }
        }
        Ok(())
    }

    pub(super) fn push(&mut self, element: Element) -> Result<(), EvaluationError> {
        self.evaluation_history.push(EvaluationOperation::Push);
        self.stack.push(element);
        Ok(())
    }

    pub(super) fn pop(&mut self) -> Result<Element, EvaluationError> {
        if self.stack.is_empty() {
            Err(EvaluationError::EmptyStack)
        } else {
            let top = self.stack.pop().unwrap();
            self.evaluation_history.push(EvaluationOperation::Pop(top.clone()));
            Ok(top)
        }
    }

    pub fn stack_len(&self) -> usize {
        self.stack.len()
    }

    pub fn stack(&self) -> impl Iterator<Item = &Element> {
        self.stack.iter()
    }
}

enum EvaluationOperation {
    Push,
    Pop(Element),
}

#[derive(Debug, PartialEq)]
pub enum EvaluationError {
    Parser(ParserError),
    EmptyStack,
    Element(ElementError),
    DivisionByZero,
}

impl Display for EvaluationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl Error for EvaluationError {
}

impl From<ParserError> for EvaluationError {
    fn from(value: ParserError) -> Self {
        EvaluationError::Parser(value)
    }
}

impl From<ElementError> for EvaluationError {
    fn from(value: ElementError) -> Self {
        EvaluationError::Element(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;

    #[test]
    fn evaluates_a_simple_function() {
        let mut env = Environment::new();
        let result = env.evaluate("1 2 add");
        assert_matches!(result, Ok(()));
        assert_eq!(env.stack.len(), 1);
        assert_matches!(env.stack[0], Element::Integer(3));
    }

    #[test]
    fn an_error_during_evaluation_does_not_change_stack() {
        let mut env = Environment::new();
        assert_matches!(env.evaluate("1 2 3"), Ok(()));
        assert_matches!(env.evaluate("+ + +"), Err(EvaluationError::EmptyStack));
        assert_eq!(env.stack, vec![Element::Integer(1), Element::Integer(2), Element::Integer(3)]);
    }
}