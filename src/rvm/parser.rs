use std::io::{BufReader,BufRead};
use std::fs::File;

use super::*;

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
	fn build_instruction(mut self) -> Result<Instruction, Error> {
		if let Some(opcode) = self.opcode_hex {
			self.instruction = (opcode as u16) << 12;
		} else { return Err(Error::ParseNoOpcodeError) }

		if let Some(target) = self.target_hex {
			self.instruction += (target as u16) << 8;
		} else { return Err(Error::ParseNoTargetError) }

		if let Some(value) = self.value_hex {
			self.instruction += value as u16;
		} else {
			match self.target_hex {
				Some(HLT) => return Ok(self.instruction),
				Some(SYS) => return Ok(self.instruction),
				_ => return Err(Error::ParseNoValueError)
			}
		}
		
		Ok(self.instruction)
	}

	fn build_sloc(self) -> Result<Instruction, Error> {
		unimplemented!()
	}
	
}


fn disassemble_line(line: Instruction) -> Sloc {
	unimplemented!()
}


fn assemble_line(line: &str) -> Result<Instruction, Error> {
	let tokens: Vec<&str> = line.split(" ").collect();
	
	let mut instruction_builder = InstructionBuilder::default();	

	match tokens.get(0).map(|s| *s) {
		Some("int") => instruction_builder.opcode_hex = Some(INT), 
		Some("set") => instruction_builder.opcode_hex = Some(SET), 
		Some("sto") => instruction_builder.opcode_hex = Some(STO), 
		Some("add") => instruction_builder.opcode_hex = Some(ADD), 
		Some("sub") => instruction_builder.opcode_hex = Some(SUB), 
		Some("mul") => instruction_builder.opcode_hex = Some(MUL), 
		Some("div") => instruction_builder.opcode_hex = Some(DIV), 
		Some("chk") => instruction_builder.opcode_hex = Some(CHK), 
		Some("cns") => instruction_builder.opcode_hex = Some(CNS), 
		Some("lpt") => instruction_builder.opcode_hex = Some(LPT), 
		Some("lsh") => instruction_builder.opcode_hex = Some(LSH), 
		Some("rsh") => instruction_builder.opcode_hex = Some(RSH), 
		Some("and") => instruction_builder.opcode_hex = Some(AND), 
		Some("bor") => instruction_builder.opcode_hex = Some(BOR), 
		Some("xor") => instruction_builder.opcode_hex = Some(XOR), 
		_ => return Err(Error::ParseNoOpcodeError)
	}

	match tokens.get(1).map(|s| *s) {
		Some("r0") => instruction_builder.target_hex = Some(R0),
		Some("r1") => instruction_builder.target_hex = Some(R1),
		Some("r2") => instruction_builder.target_hex = Some(R2),
		Some("r3") => instruction_builder.target_hex = Some(R3),
		Some("r4") => instruction_builder.target_hex = Some(R4),
		Some("r5") => instruction_builder.target_hex = Some(R5),
		Some("r6") => instruction_builder.target_hex = Some(R6),
		Some("r7") => instruction_builder.target_hex = Some(R7),
		Some("rn") => instruction_builder.target_hex = Some(RN),
		Some("rd") => instruction_builder.target_hex = Some(RD),
		Some("rf") => instruction_builder.target_hex = Some(RF),
		Some("rc") => instruction_builder.target_hex = Some(RC),
		Some("hlt") => instruction_builder.target_hex = Some(HLT),
		Some("sys") => instruction_builder.target_hex = Some(SYS),
		_ => return Err(Error::ParseNoTargetError)
	}
	
	match tokens.get(2).map(|s| *s) {
		Some("r0") => instruction_builder.value_hex = Some(R0),
		Some("r1") => instruction_builder.value_hex = Some(R1),
		Some("r2") => instruction_builder.value_hex = Some(R2),
		Some("r3") => instruction_builder.value_hex = Some(R3),
		Some("r4") => instruction_builder.value_hex = Some(R4),
		Some("r5") => instruction_builder.value_hex = Some(R5),
		Some("r6") => instruction_builder.value_hex = Some(R6),
		Some("r7") => instruction_builder.value_hex = Some(R7),
		Some("rn") => instruction_builder.value_hex = Some(RN),
		Some("rd") => instruction_builder.value_hex = Some(RD),
		Some("rf") => instruction_builder.value_hex = Some(RF),
		Some("rc") => instruction_builder.value_hex = Some(RC),
		Some(x) => instruction_builder.value_hex = x.parse::<u8>().ok(),
		//Some(error) => return Err(Error::ParseNoValueError),
		_ => {}
	}

	instruction_builder.build_instruction()
}

pub fn assemble_file(path: &str) -> Result<Bytecode, Error> {
	let mut bytecode: Bytecode = Vec::new();

	let file = File::open(path).unwrap();
       	for (linenumber, line) in BufReader::new(file).lines().enumerate() {
		let line = line.unwrap();
		let line = line.trim();
		if line.starts_with('#') || line.is_empty() { continue };
		match assemble_line(line) {
			Ok(instruction) => { bytecode.push(instruction); println!("\t{}:\t0x{:4x}\t#{}", linenumber, instruction, line) },
			Err(error) => {
				match error {
					Error::ParseNoOpcodeError => println!("Error at line {}: {}\n\t-> Hint: Invalid opcode", linenumber, line),
					Error::ParseNoTargetError => println!("Error at line {}: {}\n\t-> Hint: Must be a register, \"hlt\", or \"sys\"", linenumber, line),
					Error::ParseNoValueError => println!("Error at line {}: {}\n\t-> Hint: Must be a register (8bit integer in case of \"set\")", linenumber, line),
					_ => println!("Unknown error at line number {}: {:?}", linenumber, line)
				};
				return Err(error)
			}
		};
       	};

	Ok(bytecode)
}
