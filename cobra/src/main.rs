use sexp::*;
use sexp::Atom::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::panic;

// Task 1: Update AST Definition
#[derive(Debug, Clone)]
enum Expr {
    Num(i32),
    Bool(bool),
    Var(String),
    Let(Vec<(String, Expr)>, Box<Expr>),
    Set(String, Box<Expr>),
    Block(Vec<Expr>),
    Loop(Box<Expr>),
    Break(Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    UnOp(UnOp, Box<Expr>),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
enum UnOp {
    Add1,
    Sub1,
    Negate,
    IsNum,
    IsBool,
}

#[derive(Debug, Clone)]
enum BinOp {
    Plus,
    Minus,
    Times,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
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
//   | (isnum <expr>)
//   | (isbool <expr>)
//   | (set! <identifier> <expr>)
//   | (block <expr>+)
//   | (loop <expr>)
//   | (break <expr>)
//   | (if <expr> <expr> <expr>)
//   | (+ <expr> <expr>)
//   | (- <expr> <expr>)
//   | (* <expr> <expr>)
//   | (< <expr> <expr>)
//   | (> <expr> <expr>)
//   | (<= <expr> <expr>)
//   | (>= <expr> <expr>)
//   | (= <expr> <expr>)

// <identifier> := [a-zA-Z][a-zA-Z0-9]*  (but not reserved words)

fn parse_expr(s: &Sexp) -> Expr {
    match s {
        // <number>
        Sexp::Atom(I(n)) => Expr::Num(i32::try_from(*n).unwrap()),
        // <identifier>
        Sexp::Atom(S(name)) => {
            if name == "true" {
                return Expr::Bool(true);
            }
            if name == "false" {
                return Expr::Bool(false);
            }
            if name == "let"
                || name == "add1"
                || name == "sub1"
                || name == "negate"
                || name == "isnum"
                || name == "isbool"
                || name == "set!"
                || name == "block"
                || name == "loop"
                || name == "break"
                || name == "if"
                || name == "<"
                || name == ">"
                || name == "<="
                || name == ">="
                || name == "="
            {
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
            // (isnum <expr>)
            [Sexp::Atom(S(op)), e] if op == "isnum" => {
                Expr::UnOp(UnOp::IsNum, Box::new(parse_expr(e)))
            },
            // (isbool <expr>)
            [Sexp::Atom(S(op)), e] if op == "isbool" => {
                Expr::UnOp(UnOp::IsBool, Box::new(parse_expr(e)))
            },
            // (set! <identifier> <expr>)
            [Sexp::Atom(S(op)), Sexp::Atom(S(name)), expr] if op == "set!" => {
                Expr::Set(name.to_string(), Box::new(parse_expr(expr)))
            },
            // (block <expr>+)
            [Sexp::Atom(S(op)), exprs @ ..] if op == "block" => {
                if exprs.is_empty() {
                    panic!("Invalid block: expected at least one expression");
                }
                Expr::Block(exprs.iter().map(parse_expr).collect())
            },
            // (loop <expr>)
            [Sexp::Atom(S(op)), expr] if op == "loop" => {
                Expr::Loop(Box::new(parse_expr(expr)))
            },
            // (break <expr>)
            [Sexp::Atom(S(op)), expr] if op == "break" => {
                Expr::Break(Box::new(parse_expr(expr)))
            },
            // (if <expr> <expr> <expr>)
            [Sexp::Atom(S(op)), condition, then_expr, else_expr] if op == "if" => {
                Expr::If(
                    Box::new(parse_expr(condition)),
                    Box::new(parse_expr(then_expr)),
                    Box::new(parse_expr(else_expr)),
                )
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
            // (< <expr> <expr>)
            [Sexp::Atom(S(op)), e1, e2] if op == "<" => {
                Expr::BinOp(BinOp::Less, Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
            },
            // (> <expr> <expr>)
            [Sexp::Atom(S(op)), e1, e2] if op == ">" => {
                Expr::BinOp(BinOp::Greater, Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
            },
            // (<= <expr> <expr>)
            [Sexp::Atom(S(op)), e1, e2] if op == "<=" => {
                Expr::BinOp(BinOp::LessEqual, Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
            },
            // (>= <expr> <expr>)
            [Sexp::Atom(S(op)), e1, e2] if op == ">=" => {
                Expr::BinOp(BinOp::GreaterEqual, Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
            },
            // (= <expr> <expr>)
            [Sexp::Atom(S(op)), e1, e2] if op == "=" => {
                Expr::BinOp(BinOp::Equal, Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
            },

            _ => panic!("Invalid expression: {:?}", vec),
        },
        //       For add1: [Sexp::Atom(S(op)), e] if op == "add1" => ...
        
        _ => panic!("Invalid expression: {:?}", s),
    }
}

fn check_number(reg: &str) -> String {
    format!(
        "mov rcx, {reg}
  and rcx, 1
  cmp rcx, 0
  jne error"
    )
}

fn check_boolean(reg: &str) -> String {
    format!(
        "mov rcx, {reg}
  and rcx, 1
  cmp rcx, 1
  jne error"
    )
}

fn check_same_type(reg1: &str, reg2: &str) -> String {
    format!(
        "mov rcx, {reg1}
  xor rcx, {reg2}
  and rcx, 1
  cmp rcx, 0
  jne error"
    )
}

fn new_label(label_counter: &mut i32, name: &str) -> String {
    *label_counter += 1;
    format!("{}_{}", name, label_counter)
}

/// Task 3: Implement Code Generation
fn compile_expr(
    e: &Expr,
    env: &HashMap<String, i32>,
    stack_offset: i32,
    break_target: Option<&String>,
    label_counter: &mut i32,
) -> String {
    match e {
        Expr::Num(n) => {
            format!("mov rax, {}\nsal rax, 1", n)
        }
        Expr::Bool(true) => "mov rax, 3".to_string(),
        Expr::Bool(false) => "mov rax, 1".to_string(),

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
                instrs.push(compile_expr(expr, &env, current_offset, break_target, label_counter));

                // Store the result in the environment
                instrs.push(format!("mov [rsp - {}], rax", current_offset));

                // Update the environment
                new_env.insert(name.clone(), current_offset);

                current_offset += 8;
            }

            // Compile the body
            instrs.push(compile_expr(body, &new_env, current_offset, break_target, label_counter));

            instrs.join("\n  ")
        },

        Expr::Set(name, expr) => {
            let offset = match env.get(name) {
                Some(offset) => *offset,
                None => panic!("Unbounded variable: {}", name),
            };
            let mut instrs = Vec::new();

            instrs.push(compile_expr(expr, env, stack_offset, break_target, label_counter));
            instrs.push(format!("mov [rsp - {}], rax", offset));

            instrs.join("\n")
        },

        Expr::Block(exprs) => {
            let mut instrs = Vec::new();

            for expr in exprs {
                instrs.push(compile_expr(expr, env, stack_offset, break_target, label_counter));
            }

            instrs.join("\n")
        },

        Expr::Loop(expr) => {
            let loop_start = new_label(label_counter, "loop_start");
            let loop_end = new_label(label_counter, "loop_end");
            let mut instrs = Vec::new();

            instrs.push(format!("{}:", loop_start));
            instrs.push(compile_expr(expr, env, stack_offset, Some(&loop_end), label_counter));
            instrs.push(format!("jmp {}", loop_start));
            instrs.push(format!("{}:", loop_end));

            instrs.join("\n")
        },

        Expr::Break(expr) => {
            let target = match break_target {
                Some(target) => target,
                None => panic!("break used outside of loop"),
            };
            let mut instrs = Vec::new();

            instrs.push(compile_expr(expr, env, stack_offset, break_target, label_counter));
            instrs.push(format!("jmp {}", target));

            instrs.join("\n")
        },

        Expr::If(condition, then_expr, else_expr) => {
            let else_label = new_label(label_counter, "if_else");
            let done_label = new_label(label_counter, "if_done");
            let mut instrs = Vec::new();

            // Evaluate the condition and ensure it produced a boolean.
            instrs.push(compile_expr(condition, env, stack_offset, break_target, label_counter));
            instrs.push(check_boolean("rax"));

            // false is tagged as 1, so jump to the else branch in that case.
            instrs.push("cmp rax, 1".to_string());
            instrs.push(format!("je {}", else_label));

            instrs.push(compile_expr(then_expr, env, stack_offset, break_target, label_counter));
            instrs.push(format!("jmp {}", done_label));

            instrs.push(format!("{}:", else_label));
            instrs.push(compile_expr(else_expr, env, stack_offset, break_target, label_counter));
            instrs.push(format!("{}:", done_label));

            instrs.join("\n")
        },

        Expr::UnOp(op, subexpr) => {
            let expr_instrs = compile_expr(subexpr, env, stack_offset, break_target, label_counter);
            match op {
                UnOp::Add1 | UnOp::Sub1 | UnOp::Negate => {
                    let check_instrs = check_number("rax");
                    let op_instr = match op {
                        UnOp::Add1 => "add rax, 2",
                        UnOp::Sub1 => "sub rax, 2",
                        UnOp::Negate => "imul rax, -1",
                        _ => unreachable!(),
                    };
                    format!("{}\n{}\n{}", expr_instrs, check_instrs, op_instr)
                }
                UnOp::IsNum => {
                    let true_label = new_label(label_counter, "isnum_true");
                    let done_label = new_label(label_counter, "isnum_done");
                    format!(
                        "{}\nmov rbx, rax\nand rbx, 1\ncmp rbx, 0\nje {}\nmov rax, 1\njmp {}\n{}:\nmov rax, 3\n{}:",
                        expr_instrs, true_label, done_label, true_label, done_label
                    )
                }
                UnOp::IsBool => {
                    let true_label = new_label(label_counter, "isbool_true");
                    let done_label = new_label(label_counter, "isbool_done");
                    format!(
                        "{}\ncmp rax, 1\nje {}\ncmp rax, 3\nje {}\nmov rax, 1\njmp {}\n{}:\nmov rax, 3\n{}:",
                        expr_instrs, true_label, true_label, done_label, true_label, done_label
                    )
                }
            }
        },

        Expr::BinOp(op, e1, e2) => {
            let mut instrs = Vec::new();
            
            // Evaluate left operand
            instrs.push(compile_expr(e1, env, stack_offset, break_target, label_counter));
            
            // Save left operand on stack
            instrs.push(format!("mov [rsp - {}], rax", stack_offset));
            
            // Evaluate right operand
            instrs.push(compile_expr(e2, env, stack_offset + 8, break_target, label_counter));
            
            // Perform operation
            match op {
                BinOp::Plus | BinOp::Minus | BinOp::Times => {
                    instrs.push(check_number("rax"));
                    instrs.push(format!("mov rbx, [rsp - {}]", stack_offset));
                    instrs.push(check_number("rbx"));
                    instrs.push("sar rax, 1".to_string());
                    instrs.push("sar rbx, 1".to_string());

                    match op {
                        BinOp::Plus => {
                            instrs.push("add rax, rbx".to_string());
                        }
                        BinOp::Minus => {
                            instrs.push("sub rbx, rax".to_string());
                            instrs.push("mov rax, rbx".to_string());
                        }
                        BinOp::Times => {
                            instrs.push("imul rax, rbx".to_string());
                        }
                        _ => unreachable!(),
                    }

                    instrs.push("sal rax, 1".to_string());
                }
                BinOp::Less | BinOp::Greater | BinOp::LessEqual | BinOp::GreaterEqual => {
                    instrs.push(check_number("rax"));
                    instrs.push(format!("mov rbx, [rsp - {}]", stack_offset));
                    instrs.push(check_number("rbx"));
                    let true_label = new_label(label_counter, "cmp_true");
                    let done_label = new_label(label_counter, "cmp_done");
                    instrs.push(format!("cmp [rsp - {}], rax", stack_offset));

                    let jump = match op {
                        BinOp::Less => "jl",
                        BinOp::Greater => "jg",
                        BinOp::LessEqual => "jle",
                        BinOp::GreaterEqual => "jge",
                        _ => unreachable!(),
                    };

                    instrs.push(format!("{} {}", jump, true_label));
                    instrs.push("mov rax, 1".to_string());
                    instrs.push(format!("jmp {}", done_label));
                    instrs.push(format!("{}:", true_label));
                    instrs.push("mov rax, 3".to_string());
                    instrs.push(format!("{}:", done_label));
                }
                BinOp::Equal => {
                    let true_label = new_label(label_counter, "eq_true");
                    let done_label = new_label(label_counter, "eq_done");
                    instrs.push(format!("mov rbx, [rsp - {}]", stack_offset));
                    instrs.push(check_same_type("rbx", "rax"));
                    instrs.push(format!("cmp [rsp - {}], rax", stack_offset));
                    instrs.push(format!("je {}", true_label));
                    instrs.push("mov rax, 1".to_string());
                    instrs.push(format!("jmp {}", done_label));
                    instrs.push(format!("{}:", true_label));
                    instrs.push("mov rax, 3".to_string());
                    instrs.push(format!("{}:", done_label));
                }
            }

            instrs.join("\n  ")
        }
    }
}

fn install_compiler_error_hook() {
    panic::set_hook(Box::new(|info| {
        let message = if let Some(message) = info.payload().downcast_ref::<&str>() {
            *message
        } else if let Some(message) = info.payload().downcast_ref::<String>() {
            message.as_str()
        } else {
            "unknown panic"
        };

        eprintln!("compiler error: {}", message);
    }));
}

fn try_main() -> std::io::Result<()> {
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
    let mut label_counter = 0;
    
    // Generate assembly instructions
    let instrs = compile_expr(&expr, &env, 8, None, &mut label_counter);
    
    // Wrap instructions in assembly program template
    let asm_program = format!(
        "section .text
extern snek_error
global our_code_starts_here
our_code_starts_here:
  {}
  ret

error:
  mov rdi, 1
  sub rsp, 8
  call snek_error
",
        instrs
    );

    // Write output assembly file
    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}

fn main() {
    install_compiler_error_hook();

    let result = panic::catch_unwind(try_main);
    match result {
        Ok(Ok(())) => {}
        Ok(Err(err)) => {
            eprintln!("compiler error: {}", err);
            std::process::exit(1);
        }
        Err(_) => {
            std::process::exit(1);
        }
    }
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
