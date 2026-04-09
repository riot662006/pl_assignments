use diamondback::{compile_program, install_compiler_error_hook, parse_program};
use sexp::parse;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::panic;

fn try_main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let debug_mode = args.len() >= 2 && args[1] == "--debug";

    let (in_name, out_name) = if debug_mode {
        if args.len() != 3 {
            eprintln!("Usage: {} --debug <input.snek>", args[0]);
            std::process::exit(1);
        }
        (&args[2], None)
    } else {
        if args.len() != 3 {
            eprintln!("Usage: {} <input.snek> <output.s>", args[0]);
            std::process::exit(1);
        }
        (&args[1], Some(&args[2]))
    };

    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    let sexp_input = format!("({})", &in_contents);
    if debug_mode {
        println!("Input (wrapped): {}", sexp_input);
    }

    let sexp = parse(&sexp_input)
        .unwrap_or_else(|e| panic!("Parse error: {}", e));

    if debug_mode {
        println!("Sexp: {:?}", sexp);
    }

    let ast = parse_program(&sexp);
    if debug_mode {
        println!("AST: {:?}", ast);
    }

    let asm_program = compile_program(&ast);
    if debug_mode {
        println!("Result (Assembly):");
        println!("{}", asm_program);
    } else {
        let mut out_file = File::create(out_name.unwrap())?;
        out_file.write_all(asm_program.as_bytes())?;
    }

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
