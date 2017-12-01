use std::env;
use std::process::exit;

mod rvm;

#[macro_use] extern crate log;

fn main() {
	let filepath: String;
	let bytecode: rvm::Bytecode;
	if let Some (arg1) = env::args().nth(1) {
		filepath = arg1;
        } else {
                println!("Usage: ./rvm <path_to_assembly_code>");
                exit(1);
        }

	debug!("Assembling {}", filepath);
	match rvm::parser::assemble_file(&filepath) {
		Ok(assembly) => { debug!("success!"); bytecode = assembly; },
		_ => { println!("failed to parse file {}", filepath); exit(1) }
	}

	match rvm::vm::run(bytecode) {
		Ok(context) => debug!("Execution ok\nbacktrace registers: {:?}\nbacktrace stack: {:?}", context.registers, context.stack),
		_ => println!("Error in execution")
	}
	
}
