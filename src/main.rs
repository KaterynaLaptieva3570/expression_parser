use std::collections::HashMap;
use std::io::{self, Write};

use expression_parser::{parse_formula, eval}; 

fn main() {
    print!("Введіть формулу (наприклад ROI = (R - C) / C * 100): ");
    io::stdout().flush().unwrap();
    let mut formula = String::new();
    io::stdin().read_line(&mut formula).unwrap();
    let formula = formula.trim();

    print!("Введіть змінні (наприклад R=1500 C=1000): ");
    io::stdout().flush().unwrap();
    let mut vars_input = String::new();
    io::stdin().read_line(&mut vars_input).unwrap();

    let mut vars = HashMap::new();
    for part in vars_input.replace(',', " ").split_whitespace() {
        if let Some((k, v)) = part.split_once('=') {
            if let Ok(val) = v.trim().parse::<f64>() {
                vars.insert(k.trim().to_string(), val);
            }
        }
    }

    let expr = parse_formula(formula);
    let result = eval(&expr, &vars);

    if let Some(name) = formula.split('=').next() {
        println!("{} = {}", name.trim(), result);
    } else {
        println!("Результат = {}", result);
    }
}
