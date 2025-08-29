#![cfg_attr(test, feature(assert_matches))]

mod element;
mod environment;
mod function;
mod parser;
mod procedure;

pub use environment::{Environment, EvaluationError};
