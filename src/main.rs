use std::process::exit;
use std::{fs};
#[allow(dead_code)]
#[allow(warnings)]
use regex::Regex;
use middle::semantic_analysis::{semantic_analysis, SymbolTable};
use middle::ir::{generate_ir};
use backend::x86_assembler_generator::{execute_x86, generate_assembly};
use frontend::parser::ProgramParser;
use log::{error};
use env_logger::Env;
use lalrpop_util::ParseError;
use middle::tac_printer::print_ir;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "i", long = "input")]
    input: String,
    #[structopt(short = "o", long = "output")]
    output: String,
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}

fn main() {
    let args = Cli::from_args();

    if args.verbose {
        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    } else {
        env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();
    }

    let input_file = args.input;
    let output_file = args.output;

    let program = fs::read_to_string(input_file).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut ir_instructions = Vec::new();

    match ProgramParser::new().parse(&program) {
        Ok(ast) => {
            println!("AST: {:?}", ast);
            if let Err(err) = semantic_analysis(&ast, &mut symbol_table) {
                error!("Semantic error in : {}", err);
            }
            let ircode = generate_ir(&ast);
            ir_instructions.extend(ircode);
        },
        Err(e) => match e {
            ParseError::UnrecognizedToken { token, expected } => {
                let (start, token, _) = token;
                println!("Error parsing program: Unrecognized token in position {}: '{}', expected one of: {:?}", start, token, expected);
                let context_len = 20; // Adjust as needed
                let start_pos = if start > context_len { start - context_len } else { 0 };
                let end_pos = start + context_len;
                let error_context = &program[start_pos..end_pos];
                println!("Error context: {}", error_context);
                exit(1);
            },
            ParseError::UnrecognizedEof { location, expected} => {
                println!("Error parsing program: Unexpected end of file at position {}. Expected one of: {:?}", location, expected);
                exit(1);
            },
            _ => {
                println!("Error parsing program: {:?}", e);
                exit(1);
            }
        },
    }

    for instruction in &ir_instructions {
        println!("{:?}", instruction);
    }

    let output = print_ir(&ir_instructions);
    println!("TAC: \n{}", output);
    let assembly = generate_assembly(&ir_instructions);
    println!("assembly: \n{}", assembly);

    execute_x86(&output_file).unwrap();
}
