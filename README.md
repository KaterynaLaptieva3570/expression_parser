# Expression Parser
*A Rust-based Formula Parsing and Evaluation Engine*

**Expression Parser** is a Rust application that reads, analyzes, and evaluates mathematical formulas written in plain text, using the `pest` grammar engine.

**Main Idea:**  was to develop a reliable, extensible system for parsing and evaluating mathematical expressions using formal grammar definitions.  

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

## Parsing Logic Diagram

```mermaid
flowchart TD
    A[User Input: Formula or File] --> B[Lexer & Parser (pest)]
    B --> C[Parse Tree]
    C --> D[AST Builder (parse_expr)]
    D --> E[AST Evaluation]
    E --> F[Variable Substitution]
    F --> G[Final Computation Result]
    G --> H[Output: Printed to Console]


### How it works:
- **A → B:** The input expression (e.g., `ROI = (R - C) / C * 100`) is parsed by Pest according to grammar rules.  
- **B → C:** Pest generates a *parse tree* representing the structure of the expression.  
- **C → D:** The program builds an **AST (Abstract Syntax Tree)** with `Expr` enums.  
- **D → E:** The AST is recursively evaluated.  
- **E → F:** Variable values from user input are substituted.  
- **F → G → H:** The result is computed and displayed (e.g., `ROI = 50`).  

This diagram helps readers quickly grasp how the parsing and evaluation flow from raw input to computed output.




