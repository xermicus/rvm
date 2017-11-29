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
							Ok(target) => {
								self.registers[target as usize] = decode_value(&instruction);
								return Ok(instruction)
							},
							_ => return Err(VMError::VMUnimplementedError)
						}
					},
					Ok(PSH) => {
						match decode_target(&instruction) {
							_ => return Err(VMError::VMUnimplementedError)
						}
					},
					Ok(POP) => {
						match decode_target(&instruction) {
							_ => return Err(VMError::VMUnimplementedError)
						}
					},
					Ok(ADD) => {
						match decode_target(&instruction) {
							Ok(target) => {
								if let Ok(value) = decode_value_as_register(&instruction) {
									if let Some(new_value) = self.registers[target as usize].checked_add(self.registers[value as usize]) {
										println!("register: {}\t value: {}", target, new_value);
										self.registers[target as usize] = new_value;
										return Ok(instruction)
									} else {
										return Err(VMError::VMRegisterOverflowError)
									};
								} else { 
									return Err(VMError::VMInvalidValueError)
								};
							},
							_ => return Err(VMError::VMRegisterOverflowError)
						}
					},
					Ok(SUB) => {
						match decode_target(&instruction) {
							Ok(target) => {
								if let Ok(value) = decode_value_as_register(&instruction) {
									if let Some(new_value) = self.registers[target as usize].checked_sub(self.registers[value as usize]) {
										println!("register: {}\t value: {}", target, new_value);
										self.registers[target as usize] = new_value;
										return Ok(instruction)
									} else {
										return Err(VMError::VMRegisterOverflowError)
									};
								} else { 
									return Err(VMError::VMInvalidValueError)
								};
							},
							_ => return Err(VMError::VMRegisterOverflowError)
						}
					},
					Ok(MUL) => {
						match decode_target(&instruction) {
							Ok(target) => {
								if let Ok(value) = decode_value_as_register(&instruction) {
									if let Some(new_value) = self.registers[target as usize].checked_mul(self.registers[value as usize]) {
										println!("register: {}\t value: {}", target, new_value);
										self.registers[target as usize] = new_value;
										return Ok(instruction)
									} else {
										return Err(VMError::VMRegisterOverflowError)
									};
								} else { 
									return Err(VMError::VMInvalidValueError)
								};
							},
							_ => return Err(VMError::VMRegisterOverflowError)
						}
					},
					Ok(DIV) => {
						match decode_target(&instruction) {
							Ok(target) => {
								if let Ok(value) = decode_value_as_register(&instruction) {
									if let Some(new_value) = self.registers[target as usize].checked_div(self.registers[value as usize]) {
										println!("register: {}\t value: {}", target, new_value);
										self.registers[target as usize] = new_value;
										return Ok(instruction)
									} else {
										return Err(VMError::VMRegisterOverflowError)
									};
								} else { 
									return Err(VMError::VMInvalidValueError)
								};
							},
							_ => return Err(VMError::VMRegisterOverflowError)
						}
					},
					Ok(CHK) => {},
					Ok(CNS) => {},
					Ok(LPT) => {},
					Ok(LSH) => {
						if let Ok(target) = decode_target(&instruction) {
							if let Ok(value) = decode_value_as_register(&instruction) {
								self.registers[target as usize] <<= self.registers[value as usize];
							} else {
								return Err(VMError::VMInvalidValueError)
							}
						} else {
							return Err(VMError::VMInvalidTargetError)
						}
					},
					Ok(RSH) => {
						if let Ok(target) = decode_target(&instruction) {
							if let Ok(value) = decode_value_as_register(&instruction) {
								self.registers[target as usize] >>= self.registers[value as usize];
							} else {
								return Err(VMError::VMInvalidValueError)
							}
						} else {
							return Err(VMError::VMInvalidTargetError)
						}
					},
					Ok(AND) => {
						if let Ok(target) = decode_target(&instruction) {
							if let Ok(value) = decode_value_as_register(&instruction) {
								self.registers[target as usize] &= self.registers[value as usize];
							} else {
								return Err(VMError::VMInvalidValueError)
							}
						} else {
							return Err(VMError::VMInvalidTargetError)
						}
					},
					Ok(BOR) => {
						if let Ok(target) = decode_target(&instruction) {
							if let Ok(value) = decode_value_as_register(&instruction) {
								self.registers[target as usize] |= self.registers[value as usize];
							} else {
								return Err(VMError::VMInvalidValueError)
							}
						} else {
							return Err(VMError::VMInvalidTargetError)
						}
					},
					Ok(XOR) => {
						if let Ok(target) = decode_target(&instruction) {
							if let Ok(value) = decode_value_as_register(&instruction) {
								self.registers[target as usize] ^= self.registers[value as usize];
							} else {
								return Err(VMError::VMInvalidValueError)
							}
						} else {
							return Err(VMError::VMInvalidTargetError)
						}
					},
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
			Ok(instruction) => println!("Step ok, trace registers: {:?}", context.registers),
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
					VMError::VMRegisterOverflowError => {
						println!("Error while decoding instruction\n\t-> Hint: Register overflow / underflow");
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

fn decode_value_as_register(instruction: &Instruction) -> Result<Rsize, VMError> {
	let result = (instruction & 0x00FF) as Rsize;
	if result <= 0xd {
		Ok(result)
	} else {
		Err(VMError::VMInvalidTargetError)
	}
}

fn decode_value(instruction: &Instruction) -> Rsize {
	((instruction & 0x00FF)) as Rsize
}
