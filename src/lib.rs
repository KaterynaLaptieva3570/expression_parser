//! # Expression Parser
//! 
//! A lightweight formula parsing and evaluation engine written in Rust.
//!
//! ## Grammar Overview
//! Below are all grammar rules defined in `grammar.pest`.
//!
//! ```pest
//! WHITESPACE = _{ " " | "\t" | "\n" }
//! file       = { SOI ~ expr ~ EOI }
//! expr       = { assign | sum }
//! assign     = { ident ~ "=" ~ sum }
//! sum        = { product ~ ((plus | minus) ~ product)* }
//! product    = { power ~ ((mul | div) ~ power)* }
//! power      = { atom ~ (pow ~ atom)* }
//! atom       = { number | ident | "(" ~ expr ~ ")" | summation }
//! summation  = { sigma ~ ident ~ "=" ~ number ~ "to" ~ number ~ "(" ~ expr ~ ")" }
//! ident      = @{ ASCII_ALPHA ~ (ASCII_ALPHANUMERIC | "_")* }
//! number     = @{ "-"? ~ ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? }
//! plus       = { "+" }
//! minus      = { "-" }
//! mul        = { "*" }
//! div        = { "/" }
//! pow        = { "^" }
//! sigma      = { "Σ" }
//! ```
//!
//! ## Rule Descriptions
//! - **`WHITESPACE`** — skips spaces, tabs, and newlines.  
//! - **`file`** — top-level parser entry (`SOI ~ expr ~ EOI`).  
//! - **`expr`** — either an assignment or arithmetic expression.  
//! - **`assign`** — variable assignment, e.g. `x = 2 + 3`.  
//! - **`sum`** — addition/subtraction sequence, e.g. `2 + 3 - 1`.  
//! - **`product`** — multiplication/division, e.g. `2 * 3 / 4`.  
//! - **`power`** — exponentiation (right-associative).  
//! - **`atom`** — atomic expressions: number, variable, parentheses, or Σ-summation.  
//! - **`summation`** — mathematical Σ notation (`Σi=1to3(i^2)`).  
//! - **`ident`** — valid identifier name (starts with letter, may include digits/underscores).  
//! - **`number`** — integer or float (optionally negative).  
//! - **Operators** — `+`, `-`, `*`, `/`, `^`, and `Σ`.
//!

use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

/// Parser generated from `grammar.pest`.
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

/// AST node representing a parsed expression.
#[derive(Debug)]
pub enum Expr {
    /// Number
    Num(f64),
    /// Variable
    Var(String),
    /// a + b
    Add(Box<Expr>, Box<Expr>),
    /// a - b
    Sub(Box<Expr>, Box<Expr>),
    /// a * b
    Mul(Box<Expr>, Box<Expr>),
    /// a / b
    Div(Box<Expr>, Box<Expr>),
    /// a ^ b
    Pow(Box<Expr>, Box<Expr>),
    /// Σi=1toN(body)
    Sum {
        var: String,
        start: f64,
        end: f64,
        body: Box<Expr>,
    },
    /// a = b + c
    Assign(Box<Expr>),
}

/// Builds an [`Expr`] tree from a parsed pair.
pub fn parse_expr(pair: pest::iterators::Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::file | Rule::expr => parse_expr(pair.into_inner().next().unwrap()),
        Rule::assign => {
            let mut it = pair.into_inner();
            it.next();
            Expr::Assign(Box::new(parse_expr(it.next().unwrap())))
        }
        Rule::sum => {
            let mut it = pair.into_inner();
            let mut e = parse_expr(it.next().unwrap());
            while let Some(op) = it.next() {
                let rhs = parse_expr(it.next().unwrap());
                e = match op.as_rule() {
                    Rule::plus => Expr::Add(Box::new(e), Box::new(rhs)),
                    Rule::minus => Expr::Sub(Box::new(e), Box::new(rhs)),
                    _ => e,
                };
            }
            e
        }
        Rule::product => {
            let mut it = pair.into_inner();
            let mut e = parse_expr(it.next().unwrap());
            while let Some(op) = it.next() {
                let rhs = parse_expr(it.next().unwrap());
                e = match op.as_rule() {
                    Rule::mul => Expr::Mul(Box::new(e), Box::new(rhs)),
                    Rule::div => Expr::Div(Box::new(e), Box::new(rhs)),
                    _ => e,
                };
            }
            e
        }
        Rule::power => {
            let mut it = pair.into_inner();
            let mut e = parse_expr(it.next().unwrap());
            while let Some(op) = it.next() {
                let rhs = parse_expr(it.next().unwrap());
                if op.as_rule() == Rule::pow {
                    e = Expr::Pow(Box::new(e), Box::new(rhs));
                }
            }
            e
        }
        Rule::summation => {
            let mut it = pair.into_inner();
            let _sigma = it.next().unwrap(); // Σ
            let var = it.next().unwrap().as_str().to_string();
            let start = it.next().unwrap().as_str().parse().unwrap();
            let end = it.next().unwrap().as_str().parse().unwrap();
            let body = parse_expr(it.next().unwrap());
            Expr::Sum {
                var,
                start,
                end,
                body: Box::new(body),
            }
        }
        Rule::atom => parse_expr(pair.into_inner().next().unwrap()),
        Rule::number => Expr::Num(pair.as_str().parse().unwrap()),
        Rule::ident => Expr::Var(pair.as_str().to_string()),
        _ => Expr::Num(0.0),
    }
}

/// Parses a formula string into an AST.
pub fn parse_formula(expr_str: &str) -> Expr {
    let pair = Grammar::parse(Rule::file, expr_str)
        .unwrap()
        .next()
        .unwrap();
    parse_expr(pair)
}

/// Evaluates an expression using variable values.
pub fn eval(e: &Expr, vars: &HashMap<String, f64>) -> f64 {
    match e {
        Expr::Num(x) => *x,
        Expr::Var(v) => *vars.get(v).unwrap_or(&0.0),
        Expr::Add(a, b) => eval(a, vars) + eval(b, vars),
        Expr::Sub(a, b) => eval(a, vars) - eval(b, vars),
        Expr::Mul(a, b) => eval(a, vars) * eval(b, vars),
        Expr::Div(a, b) => eval(a, vars) / eval(b, vars),
        Expr::Pow(a, b) => eval(a, vars).powf(eval(b, vars)),
        Expr::Sum {
            var,
            start,
            end,
            body,
        } => {
            let mut sum = 0.0;
            let mut new_vars = vars.clone();
            let s = *start as i64;
            let e = *end as i64;
            for i in s..=e {
                new_vars.insert(var.clone(), i as f64);
                sum += eval(body, &new_vars);
            }
            sum
        }
        Expr::Assign(v) => eval(v, vars),
    }
}
