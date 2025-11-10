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

#[test]
fn test_power_operation() {
    let expr = parse_formula("2 ^ 3");
    let vars = HashMap::new();
    let result = eval(&expr, &vars);
    assert_eq!(result, 8.0);
}

#[test]
fn test_summation_basic() {
    let expr = parse_formula("Σk=1to3(k^2)");
    let vars = HashMap::new();
    let result = eval(&expr, &vars);
    // 1² + 2² + 3² = 14
    assert_eq!(result, 14.0);
}

#[test]
fn test_summation_with_expression() {
    let expr = parse_formula("Σi=1to4(i^2 + 1)");
    let vars = HashMap::new();
    let result = eval(&expr, &vars);
    // (1²+1) + (2²+1) + (3²+1) + (4²+1) = 2 + 5 + 10 + 17 = 34
    assert_eq!(result, 34.0);
}