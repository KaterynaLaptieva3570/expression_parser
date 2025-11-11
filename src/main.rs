use clap::{Parser, Subcommand};
use expression_parser::{eval, parse_formula};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

#[derive(Parser)]
#[command(author, version, about = "Expression Parser — CLI for parsing and evaluating mathematical formulas", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse and evaluate a formula
    Parse {
        /// Path to the file containing the formula and variables
        file: String,
    },
    /// Show project credits
    Credits,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Parse { file }) => {
            let content = fs::read_to_string(&file).expect("Не вдалося прочитати файл");

            // Розділяємо рядки: перша — формула, решта — змінні
            let mut lines = content.lines().filter(|l| !l.trim().is_empty());
            let formula = lines.next().expect("Немає формули в файлі").trim();

            // Зчитування змінних
            let mut vars = HashMap::new();
            for line in lines {
                if let Some((k, v)) = line.split_once('=')
                    && let Ok(val) = v.trim().parse::<f64>()
                {
                    vars.insert(k.trim().to_string(), val);
                }
            }

            println!("Формула з файлу:\n{}", formula);

            let expr = parse_formula(formula);
            let result = eval(&expr, &vars);

            if let Some(name) = formula.split('=').next() {
                println!("{} = {}", name.trim(), result);
            } else {
                println!("Результат = {}", result);
            }
        }

        Some(Commands::Credits) => {
            println!("Expression Parser © 2025 Kateryna Laptieva");
            println!("Розроблено як демонстрацію граматичного парсингу в Rust (pest).");
        }

        None => {
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
                if let Some((k, v)) = part.split_once('=')
                    && let Ok(val) = v.trim().parse::<f64>()
                {
                    vars.insert(k.trim().to_string(), val);
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
    }
}
