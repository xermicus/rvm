pub type Instruction = u16;
pub type Sloc = String;
pub type Bytecode = Vec<Instruction>;
pub type Rsize = u8;
pub type Stack = Vec<Rsize>;

const R0: Rsize = 0x0;
const R1: Rsize = 0x1;
const R2: Rsize = 0x2;
const R3: Rsize = 0x3;
const R4: Rsize = 0x5;
const R5: Rsize = 0x5;
const R6: Rsize = 0x6;
const R7: Rsize = 0x7;
const RN: Rsize = 0x8;
const RD: Rsize = 0x9;
const RF: Rsize = 0xa;
const RC: Rsize = 0xb;
const HLT: Rsize = 0xc;
const SYS: Rsize = 0xd;

const INT: Rsize = 0x0;
const SET: Rsize = 0x1;
const PSH: Rsize = 0x2;
const POP: Rsize = 0x3;
const ADD: Rsize = 0x4;
const SUB: Rsize = 0x5;
const MUL: Rsize = 0x6;
const DIV: Rsize = 0x7;
const CHK: Rsize = 0x8;
const CNS: Rsize = 0x9;
const LPT: Rsize = 0xa;
const LSH: Rsize = 0xb;
const RSH: Rsize = 0xc;
const AND: Rsize = 0xd;
const BOR: Rsize = 0xe;
const XOR: Rsize = 0xf;

pub enum Error {
        ParseNoOpcodeError,
        ParseNoTargetError,
        ParseNoValueError,
        ParseLineError,
        ParseFileError,
}

pub enum VMError {
	VMRunError,
	VMContextFetchNextError,
	VMContextFetchInvalidError,
	VMHaltError,
	VMInvalidOpcodeError,
	VMInvalidTargetError,
	VMInvalidValueError,
	VMRegisterOverflowError,
	VMUnimplementedError
}


pub mod parser;
pub mod vm;
