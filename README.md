# Expression Parser
*A Rust-based Formula Parsing and Evaluation Engine*

**Expression Parser** is a Rust application that reads, analyzes, and evaluates mathematical formulas written in plain text, using the `pest` grammar engine.

**Main Idea:**  was to develop a reliable, extensible system for parsing and evaluating mathematical expressions using formal grammar definitions.  

**Crates.io:** https://crates.io/crates/expression_parser/0.1.0

## Overview

This project demonstrates how to build a simple expression language parser in Rust:
- It reads a user-provided expression as a string.
- Uses a formal grammar (`.pest` file) to tokenize and parse it.
- Constructs an Abstract Syntax Tree (AST) from the parsed tokens.
- Recursively evaluates the AST using variable values from user input.
- Supports both interactive and CLI execution modes.

## Features

- Automatic substitution of variable values 
- Command-line interface for dynamic inputs
- Support for summations, parentheses, and power operations  

## Program supports:

- Numeric constants: 42, -3.14, 0.05
- Variables: x, y, Revenue, a1
- Parentheses: (a + b) * 2
- Assignments: result = (a + b) * c
- Whitespace flexibility: expressions can contain or omit spaces (a=1+2, a = 1 + 2, etc.)

## Supported Operations:

- Addition          +
- Subtraction	    -	
- Multiplication	*	
- Division	        /	
- Parentheses	    ( )	
- Exponentiation	^   
- Summation         Σ   

## Example Usage

Enter expression: result = (a + b) * (c - d / 2)<br>
Enter variables: a=3 b=5 c=10 d=4<br>
result = 68

Enter expression: sum = Σk=1to2(k^a)<br>
Enter variables: a=2<br>
// 1^2 + 2^2 = 5<br>
result = 5

You can store a formula and its variables in a file: formula.txt:<br>
ROI = (R - C) / C * 100<br>
R=1500<br>
C=1000

cargo run -- parse formula.txt<br>

Formula from file:<br>
ROI = (R - C) / C * 100<br>
ROI = 50

## Display Help:<br>
cargo run -- --help

## Display Credits:<br>
cargo run -- credits

## Grammar Definition

The parser uses a formal grammar written in [`pest`](https://pest.rs) syntax to define how formulas are recognized and parsed into an Abstract Syntax Tree (AST).

```pest
file      =  { SOI ~ expr ~ EOI }
expr      =  { assign | summation | sum | product }
assign    =  { ident ~ "=" ~ expr }
summation =  { "Σ" ~ ident ~ "=" ~ number ~ "to" ~ number ~ "(" ~ expr ~ ")" }
sum       =  { product ~ (("+" | "-") ~ product)* }
product   =  { power ~ (("*" | "/") ~ power)* }
power     =  { atom ~ ("^" ~ atom)* }
atom      =  { number | ident | "(" ~ expr ~ ")" }
number    =  @{ "-"? ~ ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? }
ident     =  @{ ASCII_ALPHA+ }

