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

		if let Some(value) = self.value_hex {
			self.instruction += (value as u16);
		} else {
			match self.target_hex {
				Some(0xc) => return Ok(self.instruction),
				Some(0xd) => return Ok(self.instruction),
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
		Some("set") => instruction_builder.opcode_hex = Some(2), 
		Some("sto") => instruction_builder.opcode_hex = Some(3), 
		Some("add") => instruction_builder.opcode_hex = Some(4), 
		Some("sub") => instruction_builder.opcode_hex = Some(5), 
		Some("mul") => instruction_builder.opcode_hex = Some(6), 
		Some("div") => instruction_builder.opcode_hex = Some(7), 
		Some("chk") => instruction_builder.opcode_hex = Some(8), 
		Some("cns") => instruction_builder.opcode_hex = Some(9), 
		Some("lpt") => instruction_builder.opcode_hex = Some(0xa), 
		Some("lsh") => instruction_builder.opcode_hex = Some(0xb), 
		Some("rsh") => instruction_builder.opcode_hex = Some(0xc), 
		Some("and") => instruction_builder.opcode_hex = Some(0xd), 
		Some("bor") => instruction_builder.opcode_hex = Some(0xe), 
		Some("xor") => instruction_builder.opcode_hex = Some(0xf), 
		_ => return Err(Error::ParseLineError)
	}

	match tokens.get(1).map(|s| *s) {
		Some("r0") => instruction_builder.target_hex = Some(0),
		Some("r1") => instruction_builder.target_hex = Some(1),
		Some("r2") => instruction_builder.target_hex = Some(2),
		Some("r3") => instruction_builder.target_hex = Some(3),
		Some("r4") => instruction_builder.target_hex = Some(4),
		Some("r5") => instruction_builder.target_hex = Some(5),
		Some("r6") => instruction_builder.target_hex = Some(6),
		Some("r7") => instruction_builder.target_hex = Some(7),
		Some("rn") => instruction_builder.target_hex = Some(8),
		Some("rd") => instruction_builder.target_hex = Some(9),
		Some("rf") => instruction_builder.target_hex = Some(0xa),
		Some("rc") => instruction_builder.target_hex = Some(0xb),
		Some("hlt") => instruction_builder.target_hex = Some(0xc),
		Some("sys") => instruction_builder.target_hex = Some(0xd),
		_ => return Err(Error::ParseLineError)
	}
	
	match tokens.get(2).map(|s| *s) {
		Some("r0") => instruction_builder.value_hex = Some(0),
		Some("r1") => instruction_builder.value_hex = Some(1),
		Some("r2") => instruction_builder.value_hex = Some(2),
		Some("r3") => instruction_builder.value_hex = Some(3),
		Some("r4") => instruction_builder.value_hex = Some(4),
		Some("r5") => instruction_builder.value_hex = Some(5),
		Some("r6") => instruction_builder.value_hex = Some(6),
		Some("r7") => instruction_builder.value_hex = Some(7),
		Some("rn") => instruction_builder.value_hex = Some(8),
		Some("rd") => instruction_builder.value_hex = Some(9),
		Some("rf") => instruction_builder.value_hex = Some(0xa),
		Some("rc") => instruction_builder.value_hex = Some(0xb),
		Some(x) => instruction_builder.value_hex = x.parse::<u8>().ok(),
		_ => {}
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
