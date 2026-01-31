use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

use sable::chunk::CompiledProto;
use sable::compiler;
use sable::disasm;
use sable::lexer::Lexer;
use sable::parser::Parser;
use sable::value::Value;
use sable::vm::Vm;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        repl();
        return;
    }
    match args[1].as_str() {
        "--version" => {
            println!("sable 0.5.0");
        }
        "--help" => {
            print_usage();
        }
        "--disasm" => {
            if args.len() < 3 {
                eprintln!("usage: sable --disasm <file>");
                std::process::exit(2);
            }
            run_disasm(&args[2]);
        }
        "--emit-chunk" => {
            if args.len() < 3 {
                eprintln!("usage: sable --emit-chunk <file>");
                std::process::exit(2);
            }
            emit_chunk(&args[2]);
        }
        path => {
            run_file(path);
        }
    }
}

fn print_usage() {
    println!("usage:");
    println!("  sable <file>            run a script");
    println!("  sable --disasm <file>   print the compiled bytecode");
    println!("  sable --version         print the version");
    println!("  sable                   start a read-eval loop");
}

fn compile_src(src: &str) -> Result<CompiledProto, String> {
    let tokens = Lexer::new(src).tokenize().map_err(|e| e.to_string())?;
    let program = Parser::new(tokens).parse_program().map_err(|e| e.to_string())?;
    compiler::compile(&program).map_err(|e| e.to_string())
}

fn run_file(path: &str) {
    let src = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("cannot read {}: {}", path, e);
            std::process::exit(1);
        }
    };
    let mut vm = Vm::new();
    if let Err(e) = vm.eval_str(&src) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run_disasm(path: &str) {
    let src = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("cannot read {}: {}", path, e);
            std::process::exit(1);
        }
    };
    match compile_src(&src) {
        Ok(proto) => print!("{}", disasm::disassemble_proto(&proto)),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn emit_chunk(path: &str) {
    let src = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("cannot read {}: {}", path, e);
            std::process::exit(1);
        }
    };
    match compile_src(&src) {
        Ok(proto) => {
            let bytes = sable::chunk::to_bytes(&proto);
            use std::io::Write;
            std::io::stdout().write_all(&bytes).ok();
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn show(v: Value) -> String {
    if v.is_number() {
        let n = v.as_number();
        if n.is_finite() && n == n.trunc() {
            format!("{}", n as i64)
        } else {
            format!("{}", n)
        }
    } else if v.is_bool() {
        format!("{}", v.as_bool())
    } else if v.is_nil() {
        String::from("nil")
    } else {
        String::from("<object>")
    }
}

fn repl() {
    let mut vm = Vm::new();
    let stdin = io::stdin();
    print!("sable> ");
    io::stdout().flush().ok();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        let trimmed = line.trim();
        if trimmed == "exit" || trimmed == "quit" {
            break;
        }
        if !trimmed.is_empty() {
            match vm.eval_str(&line) {
                Ok(v) => {
                    if !v.is_nil() {
                        println!("{}", show(v));
                    }
                }
                Err(e) => println!("error: {}", e),
            }
        }
        print!("sable> ");
        io::stdout().flush().ok();
    }
}
