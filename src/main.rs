use std::env;
use std::process::exit;
use std::io::{BufReader,BufRead};
use std::fs::File;

type Instruction = u16;
type Sloc = String;

struct Context {
	registers: [u8; 16],
	stack: Vec<u8>
}

enum Error {
	ParseNoOpcodeError,
	ParseNoTargetError,
	ParseNoValueError,
	ParseLineError,
	ParseFileError
}

#[derive(Default, Debug)]
struct InstructionBuilder {
	opcode_string: Option<String>,
	opcode_hex: Option<u8>,
	target_string: Option<String>,
	target_hex: Option<u8>,
	value_string: Option<String>,
	value_hex: Option<u8>,
	instruction: Instruction,
	sloc: Sloc
}

impl InstructionBuilder {
	fn build_Instruction(mut self) -> Result<Instruction, Error> {
		if let Some(opcode) = self.opcode_hex {
			self.instruction = (opcode as u16) << 12;
		} else { return Err(Error::ParseNoOpcodeError) }

		if let Some(target) = self.target_hex {
			self.instruction += (target as u16) << 8;
		} else { return Err(Error::ParseNoTargetError) }
			/*match self.opcode_hex {
				Some(0) => return Ok(self.instruction),
				_ => return Err(Error::ParseNoTargetError)
			};
		}*/

		if let Some(value) = self.value_hex {
			self.instruction += (value as u16);
		} else {
			match self.target_hex {
				Some(0) => return Ok(self.instruction),
				Some(1) => return Ok(self.instruction),
				_ => return Err(Error::ParseNoValueError)
			}
		}
		
		Ok(self.instruction)
	}

	fn build_Sloc(self) -> Result<Instruction, Error> {
		unimplemented!()
	}
	
}


fn disassemble_line(line: Instruction) -> Sloc {
	String::from("foo")
	
}




fn assemble_line(line: &str) -> Result<Instruction, Error> {
	let tokens: Vec<&str> = line.split(" ").collect();
	
	let mut instruction_builder = InstructionBuilder::default();	

	match tokens.get(0).map(|s| *s) {
		Some("int") => instruction_builder.opcode_hex = Some(0), 
		_ => return Err(Error::ParseLineError)
	}

	match tokens.get(1).map(|s| *s) {
		Some("0") => instruction_builder.target_hex = Some(0),
		_ => return Err(Error::ParseLineError)
	}

	instruction_builder.build_Instruction()
}

fn assemble_file(path: &str) -> Result<Vec<Instruction>, Error> {
	let mut code: Vec<Instruction> = Vec::new();

	let file = File::open(path).unwrap();
       	for (linenumber, line) in BufReader::new(file).lines().enumerate() {
		let line = line.unwrap();
		let line = line.trim();
		if line.starts_with('#') || line.is_empty() { continue };
		match assemble_line(line) {
			Ok(instruction) => code.push(instruction),
			Err(error) => { println!("Parse error at line number {}: {:?}", linenumber, line); return Err(error)}
		}
       	};

	Ok(code)
}

fn main() {
	if let Some (arg1) = env::args().nth(1) {
		match assemble_file(&arg1) {
			Ok(assembly) => println!("assembled: {:?}", assembly),
			Err(error) => { println!("failed to parse file {}", arg1); exit(1) }
		}
        } else {
                println!("Usage: ./rvm <path_to_assembly_code>");
                exit(1);
        }
}
