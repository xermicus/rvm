use super::*;

#[derive(Default, Debug)]
pub struct Context {
        registers: [Rsize; 16],
        stack: Stack,
	bytecode: Bytecode
}

impl Context {
	fn fetch(&mut self) -> Result<Instruction, Error> {
		let pointer_next = self.registers[RN as usize];
		if let Some(instruction) = self.bytecode.get(pointer_next as usize) {
			if let Some(next) = pointer_next.checked_add(1) {
				self.registers[RN as usize] = next;
			} else {
				if (instruction & 0xF00) >> 8 == RN as u16 {
					return Err(Error::VMContextFetchNextError)
				}
			}
			Ok(instruction.to_owned())
		} else {
			return Err(Error::VMContextFetchInvalidError)
		}
	}
}

pub fn run(bytecode: Bytecode) -> Result<Context, Error> {
	let mut context = Context::default();	
	context.bytecode = bytecode;

	loop {
		match context.fetch() {
			Ok(instruction) => println!("fetched: 0x{:4x}", instruction),
			Err(error) => { 
				let faulty_index = context.registers[RN as usize];
				match error {
					Error::VMContextFetchNextError => 
						println!("Error while fetching instruction at index 0x{:x}\n\t-> Hint: Programm too large?", faulty_index),
					Error::VMContextFetchInvalidError =>
						println!("Error while fetching instruction at index 0x{:x}\n\t-> Hint: No instruction found at this index", faulty_index),
					_ =>
						println!("Error while fetching next instruction at index 0x{:2x}\n\t-> Hint: Check your code", faulty_index),
				};
				return Err(Error::VMRunError)
			}
		};
			
	}

	Ok(Context::default())
}
