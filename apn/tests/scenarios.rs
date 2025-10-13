#![cfg_attr(test, feature(assert_matches))]

use std::assert_matches::assert_matches;
use apn::Environment;
use apn::Element;

#[test]
fn simple_scenario() {
    let mut env = Environment::new();
    assert_matches!(env.evaluate("1 2 +"), Ok(()));
    let stack = env.stack().collect::<Vec<_>>();
    assert_eq!(stack.len(), 1);
    assert_matches!(stack[0], Element::Integer(3));
}

#[test]
fn with_procedure() {
    let mut env = Environment::new();
    assert_matches!(env.evaluate("1 2 {+} ."), Ok(()));
    let stack = env.stack().collect::<Vec<_>>();
    assert_eq!(stack.len(), 1);
    assert_matches!(stack[0], Element::Integer(3));
}

#[test]
fn with_variables() {
    let mut env = Environment::new();
    assert_matches!(env.evaluate("{1 +} $inc = 2 $inc ."), Ok(()));
    let stack = env.stack().collect::<Vec<_>>();
    assert_eq!(stack.len(), 1);
    assert_matches!(stack[0], Element::Integer(3));
}

#[test]
fn with_variables_inside_procedures() {
    let mut env = Environment::new();
    assert_matches!(env.evaluate("{1 +} $inc = 2 {$inc} . ."), Ok(()));
    let stack = env.stack().collect::<Vec<_>>();
    assert_eq!(stack.len(), 1);
    assert_matches!(stack[0], Element::Integer(3));
}