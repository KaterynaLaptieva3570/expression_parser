use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

#[derive(Debug)]
pub enum Expr {
    Num(f64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Sum {
        var: String,
        start: f64,
        end: f64,
        body: Box<Expr>,
    },
    Assign(Box<Expr>),
}

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

pub fn parse_formula(expr_str: &str) -> Expr {
    let pair = Grammar::parse(Rule::file, expr_str).unwrap().next().unwrap();
    parse_expr(pair)
}

pub fn eval(e: &Expr, vars: &HashMap<String, f64>) -> f64 {
    match e {
        Expr::Num(x) => *x,
        Expr::Var(v) => *vars.get(v).unwrap_or(&0.0),
        Expr::Add(a, b) => eval(a, vars) + eval(b, vars),
        Expr::Sub(a, b) => eval(a, vars) - eval(b, vars),
        Expr::Mul(a, b) => eval(a, vars) * eval(b, vars),
        Expr::Div(a, b) => eval(a, vars) / eval(b, vars),
        Expr::Pow(a, b) => eval(a, vars).powf(eval(b, vars)),
        Expr::Sum { var, start, end, body } => {
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
