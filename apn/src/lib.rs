#![cfg_attr(test, feature(assert_matches))]

mod element;
mod environment;
mod function;
mod parser;

pub use environment::{Environment,EvaluationError};
