use std::collections::HashMap;
use pretty_assertions::assert_eq;
use expression_parser::{parse_formula, eval};

#[test]
fn test_simple_addition() {
    let expr = parse_formula("1 + 2 * 3");
    let vars = HashMap::new();
    let result = eval(&expr, &vars);
    assert_eq!(result, 7.0);
}

#[test]
fn test_parentheses_precedence() {
    let expr = parse_formula("(1 + 2) * 3");
    let vars = HashMap::new();
    let result = eval(&expr, &vars);
    assert_eq!(result, 9.0);
}

#[test]
fn test_variables_usage() {
    let expr = parse_formula("A + B * 2");
    let mut vars = HashMap::new();
    vars.insert("A".into(), 3.0);
    vars.insert("B".into(), 4.0);
    let result = eval(&expr, &vars);
    assert_eq!(result, 11.0);
}

#[test]
fn test_roi_formula() {
    let expr = parse_formula("ROI = (R - C) / C * 100");
    let mut vars = HashMap::new();
    vars.insert("R".into(), 1500.0);
    vars.insert("C".into(), 1000.0);
    let result = eval(&expr, &vars);
    assert_eq!(result, 50.0);
}

