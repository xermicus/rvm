use super::*;

#[derive(Default, Debug)]
pub struct Context {
        registers: [Rsize; 14],
        stack: Stack,
	bytecode: Bytecode
}

impl Context {
	fn fetch(&mut self) -> Result<Instruction, VMError> {
		let pointer_next = self.registers[RN as usize];
		if let Some(instruction) = self.bytecode.get(pointer_next as usize) {
			if let Some(next) = pointer_next.checked_add(1) {
				self.registers[RN as usize] = next;
			} else {
				if let Ok(register) = decode_target(instruction) {
					if register != RN {
						return Err(VMError::VMContextFetchNextError)
					};
				}
			}
			Ok(instruction.to_owned())
		} else {
			return Err(VMError::VMContextFetchInvalidError)
		}
	}

	fn step(&mut self) -> Result<Instruction, VMError> {
		match self.fetch() {
			Ok(instruction) => {
				println!("fetched: 0x{:4x}", instruction);
				match decode_opcode(&instruction) {
					Ok(INT) => {
						match decode_target(&instruction) {
							Ok(HLT) => { println!("Debug: got HLT"); return Err(VMError::VMHaltError) },
							Ok(SYS) => { println!("Debug: got SYS"); return Err(VMError::VMUnimplementedError) },
							_ => { return Err(VMError::VMInvalidTargetError) }
						}
					},
					Ok(SET) => {
						match decode_target(&instruction) {
							_ => return Err(VMError::VMUnimplementedError)
						}
					},
					Ok(PSH) => {},
					Ok(POP) => {},
					Ok(ADD) => {},
					Ok(SUB) => {},
					Ok(MUL) => {},
					Ok(DIV) => {},
					Ok(CHK) => {},
					Ok(CNS) => {},
					Ok(LPT) => {},
					Ok(LSH) => {},
					Ok(RSH) => {},
					Ok(AND) => {},
					Ok(BOR) => {},
					Ok(XOR) => {},
					_ => return Err(VMError::VMInvalidOpcodeError)
				}
				return Ok(instruction)
			},
			Err(error) => return Err(error)
		};

		return Err(VMError::VMRunError)
	}
}

pub fn run(bytecode: Bytecode) -> Result<Context, VMError> {
	let mut context = Context::default();	
	context.bytecode = bytecode;

	loop {
		match context.step()	{
			Ok(instruction) => println!("Step ok"),
			Err(error) => {
				let faulty_index = context.registers[RN as usize];
				match error {
					VMError::VMContextFetchNextError => {
						println!("Error while fetching instruction at index 0x{:x}\n\t-> Hint: Programm too large?", faulty_index);
						break
					}
					VMError::VMContextFetchInvalidError => {
						println!("Error while fetching instruction at index 0x{:x}\n\t-> Hint: No instruction found at this index", faulty_index);
						break
					}
					VMError::VMUnimplementedError => {
						println!("Error while decoding instruction\n\t-> Hint: Instruction not implemented (yet)");
						break
					}
					VMError::VMInvalidOpcodeError => {
						println!("Error while decoding instruction\n\t-> Hint: Invalid Opcode)");
						break
					}
					VMError::VMInvalidTargetError => {
						println!("Error while decoding instruction\n\t-> Hint: Invalid Target");
						break
					}
					VMError::VMInvalidValueError => {
						println!("Error while decoding instruction\n\t-> Hint: Invalid Value");
						break
					}
					VMError::VMHaltError => break,
					_ => println!("Unhandled Runtime Error")
				};
				break
			}
		}; 
	}

	Ok(Context::default())
}

fn decode_opcode(instruction: &Instruction) -> Result<Rsize, VMError> {
	let result = ((instruction & 0xF000) >> 12) as Rsize;
	if result <= 0xf {
		Ok(result)
	} else {
		Err(VMError::VMInvalidOpcodeError)
	}
}

fn decode_target(instruction: &Instruction) -> Result<Rsize, VMError> {
	let result = ((instruction & 0x0F00) >> 8) as Rsize;
	if result <= 0xd {
		Ok(result)
	} else {
		Err(VMError::VMInvalidTargetError)
	}
}

fn decode_value(instruction: &Instruction) -> Result<Rsize, VMError> {
	Ok(((instruction & 0x00FF)) as Rsize)
}
