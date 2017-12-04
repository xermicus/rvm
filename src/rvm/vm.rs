use std::cmp::Ordering;
use std::char;
use std::io;
use super::*;

#[derive(Default, Debug)]
pub struct Context {
        pub registers: [Rsize; 13],
        pub stack: Stack,
	pub bytecode: Bytecode
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
				match decode_opcode(&instruction) {
					Ok(INT) => {
						match self.registers[RS as usize] {
							HALT => return Err(VMError::VMHaltError),
							PRINTLINE => {
								let mut index = self.registers[R0 as usize] as usize;
								let mut string = String::new();
								loop {
									if let Some(raw) = self.stack.get(index as usize) {
										if let Some(chr) = char::from_u32(raw.to_owned() as u32) {
											string.push(chr);
										} else {
											break
										}
										index += 1;
									} else {
										break
										//return Err(VMError::VMStackInvalidAccessError)
									}
								}
								println!("{}", string);
							},
							READLINE => { 
								let mut input = String::new();
								let mut length = self.registers[R0 as usize] as usize;
								let mut count = 0;
								if let Ok(line) = io::stdin().read_line(&mut input) {
									input.pop();
									let mut chars = input.chars();
									while let Some(chr) = chars.next() {
										if count >= length {
											break
										}
										self.stack.push(chr as u8);
										if let Some(new_rd) = self.registers[RD as usize].checked_add(1) {
											self.registers[RD as usize] = new_rd;
										} else {
											return Err(VMError::VMStackOverflowError)
										}
										count += 1;
									};
									self.registers[R0 as usize] = count as Rsize;
									println!("R0 = {}",self.registers[R0 as usize]);
								} else {
									return Err(VMError::VMInterruptError)
								}
							},
							_ => return Err(VMError::VMUnimplementedError)
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
						if let Ok(target) = decode_target(&instruction) {
							if let Ok(value) = decode_value_as_register(&instruction) {
								for register in target.min(value)..target.max(value)+1 {
									self.stack.push(self.registers[register as usize]);
									if let Some(new_rd) = self.registers[RD as usize].checked_add(1) {
										self.registers[RD as usize] = new_rd;
									} else {
										return Err(VMError::VMStackOverflowError)
									}
								}
								debug!("PSH ok, stack: {:?}", self.stack);
							} else {
								return Err(VMError::VMInvalidValueError)
							}
						} else {
							return Err(VMError::VMInvalidTargetError)
						}
					},
					Ok(POP) => {
						if let Ok(target) = decode_target(&instruction) {
							if let Ok(value) = decode_value_as_register(&instruction) {
								for register in (target.min(value)..target.max(value)+1).rev() {
									if let Some(value) = self.stack.pop() {
										self.registers[register as usize] = value;
									} else {
										return Err(VMError::VMStackOverflowError)
									};
									if let Some(new_rd) = self.registers[RD as usize].checked_sub(1) {
										self.registers[RD as usize] = new_rd;
									} else {
										return Err(VMError::VMStackOverflowError)
									}
								}
								debug!("PSH ok, stack: {:?}", self.stack);
							} else {
								return Err(VMError::VMInvalidValueError)
							}
						} else {
							return Err(VMError::VMInvalidTargetError)
						}
					},
					Ok(ADD) => {
						match decode_target(&instruction) {
							Ok(target) => {
								if let Ok(value) = decode_value_as_register(&instruction) {
									if let Some(new_value) = self.registers[target as usize].checked_add(self.registers[value as usize]) {
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
										self.registers[target as usize] = new_value;
										return Ok(instruction)
									} else {
										return Err(VMError::VMRegisterOverflowError)
									};
								} else { 
									return Err(VMError::VMInvalidValueError)
								}
							},
							_ => return Err(VMError::VMRegisterOverflowError)
						}
					},
					Ok(MUL) => {
						match decode_target(&instruction) {
							Ok(target) => {
								if let Ok(value) = decode_value_as_register(&instruction) {
									if let Some(new_value) = self.registers[target as usize].checked_mul(self.registers[value as usize]) {
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
					Ok(CHK) => {
						if let Ok(target) = decode_target(&instruction) {
							if let Ok(value) = decode_value_as_register(&instruction) {
								match self.registers[target as usize].cmp(&self.registers[value as usize]) {
									Ordering::Equal => {
										self.registers[RF as usize] = EQ;
									},
									Ordering::Less => {
										self.registers[RF as usize] = LE;
									},
									Ordering::Greater => {
										self.registers[RF as usize] = GR;
									}
								}
							} else {
								return Err(VMError::VMInvalidValueError)
							}
						} else {
							return Err(VMError::VMInvalidTargetError)
						}
					},
					Ok(CNS) => {
						if let Ok(target) = decode_target(&instruction) {
							if let Ok(value) = decode_value_as_register(&instruction) {
								if self.registers[RF as usize] == self.registers[RC as usize] {
									self.registers[target as usize] = self.registers[value as usize];
								}
							} else {
								return Err(VMError::VMInvalidValueError)
							}
						} else {
							return Err(VMError::VMInvalidTargetError)
						}
					},
					Ok(LPT) => {
						if let Ok(target) = decode_target(&instruction) {
							if let Ok(value) = decode_value_as_register(&instruction) {
								if let Some(resolved) = self.stack.get(self.registers[value as usize] as usize) {
									self.registers[target as usize] = resolved.to_owned();
								} else {
									return Err(VMError::VMStackInvalidAccessError)
								}
							} else {
								return Err(VMError::VMInvalidValueError)
							}
						} else {
							return Err(VMError::VMInvalidTargetError)
						}
					},
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
		}
	}
}

pub fn run(bytecode: Bytecode) -> Result<Context, VMError> {
	let mut context = Context::default();	
	context.bytecode = bytecode;

	loop {
		match context.step()	{
			Ok(instruction) => debug!("Step {:x} ok, trace registers: {:?}", instruction, context.registers),
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
					VMError::VMStackOverflowError => {
						println!("Error while decoding instruction\n\t-> Hint: Stack overflow");
					}
					VMError::VMStackInvalidAccessError => {
						println!("Error while decoding instruction\n\t-> Hint: Invalid Stack access");
					}
					VMError::VMInterruptError => {
						println!("Error while decoding instruction\n\t-> Hint: Interrupt Error");
					}
					VMError::VMHaltError => break,
				};
				break
			}
		}; 
	}

	Ok(context)
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
	if result <= RS {
		Ok(result)
	} else {
		Err(VMError::VMInvalidTargetError)
	}
}

fn decode_value_as_register(instruction: &Instruction) -> Result<Rsize, VMError> {
	let result = (instruction & 0x00FF) as Rsize;
	if result <= RS {
		Ok(result)
	} else {
		Err(VMError::VMInvalidTargetError)
	}
}

fn decode_value(instruction: &Instruction) -> Rsize {
	((instruction & 0x00FF)) as Rsize
}
