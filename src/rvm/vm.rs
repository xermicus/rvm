use super::*;

#[derive(Default, Debug)]
pub struct Context {
        registers: [Rsize; 16],
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
				if decode_target(instruction) == RN {
					return Err(VMError::VMContextFetchNextError)
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
					INT => {
						match decode_target(&instruction) {
							HLT => { println!("Debug: got HLT") },
							SYS => { println!("Debug: got SYS") },
							_ => { return Err(VMError::VMRunError) }
						}
					},
					SET => {},
					STO => {},
					ADD => {},
					SUB => {},
					MUL => {},
					DIV => {},
					CHK => {},
					CNS => {},
					LPT => {},
					LSH => {},
					RSH => {},
					AND => {},
					BOR => {},
					XOR => {},
					_ => return Err(VMError::VMRunError)
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
					VMError::VMContextFetchNextError => 
						println!("Error while fetching instruction at index 0x{:x}\n\t-> Hint: Programm too large?", faulty_index),
					VMError::VMContextFetchInvalidError =>
						println!("Error while fetching instruction at index 0x{:x}\n\t-> Hint: No instruction found at this index", faulty_index),
					_ => println!("Runtime Error")
				};
				break
			}
		}; 
	}

	Ok(Context::default())
}

fn decode_opcode(instruction: &Instruction) -> Rsize {
	((instruction & 0xF000) >> 12) as Rsize
}

fn decode_target(instruction: &Instruction) -> Rsize {
	((instruction & 0x0F00) >> 8) as Rsize
}

fn decode_value(instruction: &Instruction) -> Rsize {
	((instruction & 0x00FF)) as Rsize
}
