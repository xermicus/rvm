use std::env;
use std::process::exit;

mod rvm;


fn main() {
	let mut filepath: String;
	let mut bytecode: rvm::Bytecode;
	if let Some (arg1) = env::args().nth(1) {
		filepath = arg1;
        } else {
                println!("Usage: ./rvm <path_to_assembly_code>");
                exit(1);
        }

	println!("Assembling {}", filepath);
	match rvm::parser::assemble_file(&filepath) {
		Ok(assembly) => { println!("success!"); bytecode = assembly; },
		Err(error) => { println!("failed to parse file {}", filepath); exit(1) }
	}

	rvm::vm::run(bytecode);
}
