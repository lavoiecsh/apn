use crate::element::Element;
use crate::parser::{parse, ParserError};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub struct Environment {
    stack: Vec<Element>,
    variables: HashMap<String, Element>,
    evaluation_history: Vec<EvaluationOperation>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            variables: HashMap::new(),
            evaluation_history: Vec::new(),
        }
    }

    pub fn evaluate(&mut self, input: impl Into<String>) -> Result<(), EvaluationError> {
        let elements = parse(input)?;
        if let Err(e) = self.evaluate_elements(elements) {
            for item in self.evaluation_history.iter().rev() {
                match item {
                    EvaluationOperation::Push => {
                        self.stack.pop();
                    }
                    EvaluationOperation::Pop(element) => {
                        self.stack.push(element.clone());
                    }
                }
            }
            self.evaluation_history.clear();
            Err(e)
        } else {
            self.evaluation_history.clear();
            Ok(())
        }
    }

    fn evaluate_elements(&mut self, elements: Vec<Element>) -> Result<(), EvaluationError> {
        for element in elements {
            self.push(element)?;
        }
        Ok(())
    }

    pub(super) fn push(&mut self, element: Element) -> Result<(), EvaluationError> {
        match element {
            Element::Function(f) => f.execute(self)?,
            _ => {
                self.evaluation_history.push(EvaluationOperation::Push);
                self.stack.push(element);
            }
        }
        Ok(())
    }

    pub(super) fn pop(&mut self) -> Result<Element, EvaluationError> {
        if self.stack.is_empty() {
            Err(EvaluationError::EmptyStack)
        } else {
            let top = self.stack.pop().unwrap();
            self.evaluation_history
                .push(EvaluationOperation::Pop(top.clone()));
            Ok(top)
        }
    }

    pub(super) fn pop_value(&mut self) -> Result<Element, EvaluationError> {
        let top = self.pop()?;
        self.resolve(&top)
    }

    fn resolve(&self, element: &Element) -> Result<Element, EvaluationError> {
        match element {
            Element::Variable(name) => self.resolve_variable(name),
            Element::Array(array) => Ok(Element::Array(
                array
                    .iter()
                    .map(|e| self.resolve(&e))
                    .collect::<Result<Vec<Element>, EvaluationError>>()?,
            )),
            element => Ok(element.clone()),
        }
    }

    fn resolve_variable(&self, name: &String) -> Result<Element, EvaluationError> {
        if let Some(element) = self.variables.get(name) {
            self.resolve(element)
        } else {
            Err(EvaluationError::UndefinedVariable(name.clone()))
        }
    }

    pub(super) fn assign(
        &mut self,
        variable: String,
        value: Element,
    ) -> Result<(), EvaluationError> {
        self.variables.insert(variable, value);
        Ok(())
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
    DivisionByZero,
    FunctionNotApplicable,
    UndefinedVariable(String),
    InvalidStackElements,
    CircularVariableReference,
    NotAString,
    NotACharacter,
    IO(String),
}

impl Display for EvaluationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl Error for EvaluationError {}

impl From<ParserError> for EvaluationError {
    fn from(value: ParserError) -> Self {
        EvaluationError::Parser(value)
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
        assert_eq!(
            env.stack,
            vec![
                Element::Integer(1),
                Element::Integer(2),
                Element::Integer(3)
            ]
        );
    }

    #[test]
    fn stores_array_as_variable() {
        let mut env = Environment::new();
        let result = env.evaluate("[1 2 +] $test =");
        assert_matches!(result, Ok(()));
        assert_eq!(env.stack_len(), 0);
        assert_matches!(env.variables.get("test"), Some(&Element::Array(_)));
        if let Some(Element::Array(array)) = env.variables.get("test") {
            assert_matches!(&array[0], Element::Integer(1));
            assert_matches!(&array[1], Element::Integer(2));
            assert_matches!(&array[2], Element::Function(_));
        }
    }
}
