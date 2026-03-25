use sexp::*;
use sexp::Atom::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

// Task 1: Update AST Definition
#[derive(Debug, Clone)]
enum Expr {
    Num(i32),
    Var(String),
    Let(Vec<(String, Expr)>, Box<Expr>),
    UnOp(UnOp, Box<Expr>),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
enum UnOp {
    Add1,
    Sub1,
    Negate,
}

#[derive(Debug, Clone)]
enum BinOp {
    Plus,
    Minus,
    Times,
}

// Task 2: Extend Parser
// Concrete Syntax:
// <expr> :=
//   | <number>
//   | <identifier>
//   | (let ((<identifier> <expr>)+) <expr>)
//   | (add1 <expr>)
//   | (sub1 <expr>)
//   | (negate <expr>)
//   | (+ <expr> <expr>)
//   | (- <expr> <expr>)
//   | (* <expr> <expr>)

// <identifier> := [a-zA-Z][a-zA-Z0-9]*  (but not reserved words)

fn parse_expr(s: &Sexp) -> Expr {
    match s {
        // <number>
        Sexp::Atom(I(n)) => Expr::Num(i32::try_from(*n).unwrap()),
        // <identifier>
        Sexp::Atom(S(name)) => {
            if name == "let" || name == "add1" || name == "sub1" || name == "negate" {
                panic!("Invalid use of keyword as identifier: {}", name);
            }
            Expr::Var(name.to_string())
        }
        Sexp::List(vec) => match &vec[..] {
            // (let ((<identifier> <expr>)+) <expr>)
            [Sexp::Atom(S(op)), Sexp::List(bindings), body]
                if op == "let" => {
                    let parsed_bindings: Vec<(String, Expr)> = bindings.iter().map(|binding| {
                        match binding {
                            Sexp::List(pair) => match &pair[..] {
                                [Sexp::Atom(S(id)), expr] => {
                                    (id.to_string(), parse_expr(expr))
                                },
                                _ => panic!("Invalid binding: {:?}", pair),
                            }
                            _ => panic!("Invalid binding: {:?}", binding),
                        }
                    }).collect();

                    Expr::Let(parsed_bindings, Box::new(parse_expr(body)))
                },

            // (add1 <expr>)
            [Sexp::Atom(S(op)), e] if op == "add1" => {
                Expr::UnOp(UnOp::Add1, Box::new(parse_expr(e)))
            },
            // (sub1 <expr>)
            [Sexp::Atom(S(op)), e] if op == "sub1" => {
                Expr::UnOp(UnOp::Sub1, Box::new(parse_expr(e)))
            },
            // (negate <expr>)
            [Sexp::Atom(S(op)), e] if op == "negate" => {
                Expr::UnOp(UnOp::Negate, Box::new(parse_expr(e)))
            },

            // (+ <expr> <expr>)
            [Sexp::Atom(S(op)), e1, e2] if op == "+" => {
                Expr::BinOp(BinOp::Plus, Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
            },
            // (- <expr> <expr>)
            [Sexp::Atom(S(op)), e1, e2] if op == "-" => {
                Expr::BinOp(BinOp::Minus, Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
            },
            // (* <expr> <expr>)
            [Sexp::Atom(S(op)), e1, e2] if op == "*" => {
                Expr::BinOp(BinOp::Times, Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
            },

            _ => panic!("Invalid expression: {:?}", vec),
        },
        //       For add1: [Sexp::Atom(S(op)), e] if op == "add1" => ...
        
        _ => panic!("Invalid expression: {:?}", s),
    }
}

/// Task 3: Implement Code Generation
fn compile_expr(e: &Expr, env: &HashMap<String, i32>, stack_offset: i32) -> String {
    match e {
        Expr::Num(n) => format!("mov rax, {}", n),

        Expr::Var(name) => {
            match env.get(name) {
                Some(offset) => format!("mov rax, [rsp - {}]", offset),
                None => panic!("Unbounded variable: {}", name),
            }
        },
        
        Expr::Let(bindings, body) => {
            let mut instrs = Vec::new();
            let mut new_env = env.clone();
            let mut current_offset = stack_offset;

            for (name, expr) in bindings {
                if bindings.iter().filter(|(n, _)| n == name).count() > 1 {
                    panic!("Duplicate binding: {}", name);
                }
                
                // Compile the expression
                instrs.push(compile_expr(expr, &env, current_offset));

                // Store the result in the environment
                instrs.push(format!("mov [rsp - {}], rax", current_offset));

                // Update the environment
                new_env.insert(name.clone(), current_offset);

                current_offset += 8;
            }

            // Compile the body
            instrs.push(compile_expr(body, &new_env, current_offset));

            instrs.join("\n  ")
        },

        Expr::UnOp(op, subexpr) => {
            let expr_instrs = compile_expr(subexpr, env, stack_offset);
            let op_instr = match op {
                UnOp::Add1 => "add rax, 1",
                UnOp::Sub1 => "sub rax, 1",
                UnOp::Negate => "imul rax, -1",
            };
            format!("{}\n{}", expr_instrs, op_instr)
        },

        Expr::BinOp(op, e1, e2) => {
            let mut instrs = Vec::new();
            
            // Evaluate left operand
            instrs.push(compile_expr(e1, env, stack_offset));
            
            // Save left operand on stack
            instrs.push(format!("mov [rsp - {}], rax", stack_offset));
            
            // Evaluate right operand
            instrs.push(compile_expr(e2, env, stack_offset + 8));
            
            // Perform operation
            match op {
                BinOp::Plus => {
                    instrs.push(format!("add rax, [rsp - {}]", stack_offset));
                }
                BinOp::Minus => {
                    instrs.push(format!("mov rbx, [rsp - {}]", stack_offset));
                    instrs.push("sub rbx, rax".to_string());
                    instrs.push("mov rax, rbx".to_string());
                }
                BinOp::Times => {
                    instrs.push(format!("imul rax, [rsp - {}]", stack_offset));
                }
            }
            
            instrs.join("\n  ")
        }
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: {} <input.snek> <output.s>", args[0]);
        std::process::exit(1);
    }

    let in_name = &args[1];
    let out_name = &args[2];

    // Read input file
    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    // Parse S-expression from text
    let sexp = parse(&in_contents).unwrap_or_else(|e| {
        panic!("Parse error: {}", e)
    });
    
    // Convert S-expression to our AST
    let expr = parse_expr(&sexp);

    // Start with empty environment and offset 8
    let env = HashMap::new();
    
    // Generate assembly instructions
    let instrs = compile_expr(&expr, &env, 8);
    
    // Wrap instructions in assembly program template
    let asm_program = format!(
        "section .text
global our_code_starts_here
our_code_starts_here:
  {}
  ret
",
        instrs
    );

    // Write output assembly file
    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}

// ============= TESTS (Optional but recommended) =============
// 
// Uncomment and run with: cargo test
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_parse_number() {
//         let sexp = parse("42").unwrap();
//         let expr = parse_expr(&sexp);
//         // Add your assertions here
//     }
//
//     #[test]
//     fn test_compile_number() {
//         let expr = Expr::Num(42);
//         let asm = compile_expr(&expr);
//         assert_eq!(asm, "mov rax, 42");
//     }
// }