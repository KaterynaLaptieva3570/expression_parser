use std::collections::HashMap;
use pretty_assertions::assert_eq;
use expression_parser::{parse_formula, eval};

// тест формул із пробілами
#[test]
fn test_whitespace() {
    let expr = parse_formula("  1   +   2 ");
    let vars = HashMap::new();
    let result = eval(&expr, &vars);
    assert_eq!(result, 3.0);
}

// усі формули парсяться як повний вираз
#[test]
fn test_file_rule() {
    let expr = parse_formula("1 + 2");
    let vars = HashMap::new();
    let result = eval(&expr, &vars);
    assert_eq!(result, 3.0);
}

// тест для expr
#[test]
fn test_expr_rule() {
    // форма expr з присвоєнням
    let expr_assign = parse_formula("result = 2 + 3");
    let vars = HashMap::new();
    let result_assign = eval(&expr_assign, &vars);
    assert_eq!(result_assign, 5.0);

    // форма expr без без присвоєння
    let expr_sum = parse_formula("4 + 6 / 2");
    let vars = HashMap::new();
    let result_sum = eval(&expr_sum, &vars);
    assert_eq!(result_sum, 7.0);
}

// присвоєння значення змінній
#[test]
fn test_assign_rule() {
    let expr = parse_formula("ROI = (R - C) / C * 100");
    let mut vars = HashMap::new();
    vars.insert("R".into(), 1500.0);
    vars.insert("C".into(), 1000.0);

    let result = eval(&expr, &vars);
    // ROI = (1500 - 1000) / 1000 * 100 = 50
    assert_eq!(result, 50.0);
}

// додавання й віднімання
#[test]
fn test_sum_rule() {
    let expr = parse_formula("2 + 3 - 1");
    let vars = HashMap::new();
    let result = eval(&expr, &vars);
    assert_eq!(result, 4.0);
}

// множення та ділення
#[test]
fn test_product_rule() {
    let expr = parse_formula("4 * 3 / 2");
    let vars = HashMap::new();
    let result = eval(&expr, &vars);
    // (4 * 3) / 2 = 6
    assert_eq!(result, 6.0);
}

// піднесення до степеня
#[test]
fn test_power_operation() {
    let expr = parse_formula("2 ^ 3");
    let vars = HashMap::new();
    let result = eval(&expr, &vars);
    assert_eq!(result, 8.0);
}

// перевірка всіх типів атомів у виразі
#[test]
fn test_atom_rule() {
    let mut vars = HashMap::new();
    vars.insert("x".into(), 2.0);
    vars.insert("y".into(), 3.0);

    // вираз містить:
    // - число 5
    // - змінні (x, y)
    // - дужковий підвираз (x + y)
    // - операцію множення (*)
    // - суму Σi=1to2(i)
    let expr = parse_formula("5 + (x + y) * 3 + Σi=1to2(i)");
    let result = eval(&expr, &vars);

    // Обчислення покроково:
    // (x + y) * 3 = (2 + 3) * 3 = 15
    // Σi=1to2(i) = 1 + 2 = 3
    // 5 + 15 + 3 = 23
    assert_eq!(result, 23.0);
}

// тест математичної суми Σ
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

// тест ідентифікаторів
#[test]
fn test_ident_rule() {
    // вираз містить:
    // - змінну з літер (x)
    // - змінну з великих літер (ROI)
    // - змінну з підкресленням та цифрою (Revenue_2025)
    let mut vars = HashMap::new();
    vars.insert("x".into(), 2.0);
    vars.insert("ROI".into(), 50.0);
    vars.insert("Revenue_2025".into(), 1000.0);

    let expr = parse_formula("x + ROI + Revenue_2025");
    let result = eval(&expr, &vars);

    // 2 + 50 + 1000 = 1052
    assert_eq!(result, 1052.0);
}

// перевірка розпізнавання чисел
#[test]
fn test_number_rule() {
    // вираз містить:
    // - додатне ціле (5)
    // - від’ємне число (-3)
    // - десяткове число (2.5)
    let expr = parse_formula("5 + (-3) + 2.5");
    let vars = HashMap::new();
    let result = eval(&expr, &vars);

    // 5 + (-3) + 2.5 = 4.5
    assert_eq!(result, 4.5);
}


// додаткові перевірки
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

