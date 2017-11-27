pub type Instruction = u16;
pub type Sloc = String;
pub type Bytecode = Vec<Instruction>;
pub type Rsize = u8;
pub type Stack = Vec<Rsize>;

const R0: u8 = 0x0;
const R1: u8 = 0x1;
const R2: u8 = 0x2;
const R3: u8 = 0x3;
const R4: u8 = 0x5;
const R5: u8 = 0x5;
const R6: u8 = 0x6;
const R7: u8 = 0x7;
const RN: u8 = 0x8;
const RD: u8 = 0x9;
const RF: u8 = 0xa;
const RC: u8 = 0xb;
const HLT: u8 = 0xc;
const SYS: u8 = 0xd;

const INT: u8 = 0x0;
const SET: u8 = 0x2;
const STO: u8 = 0x3;
const ADD: u8 = 0x4;
const SUB: u8 = 0x5;
const MUL: u8 = 0x6;
const DIV: u8 = 0x7;
const CHK: u8 = 0x8;
const CNS: u8 = 0x9;
const LPT: u8 = 0xa;
const LSH: u8 = 0xb;
const RSH: u8 = 0xc;
const AND: u8 = 0xd;
const BOR: u8 = 0xe;
const XOR: u8 = 0xf;

pub enum Error {
        ParseNoOpcodeError,
        ParseNoTargetError,
        ParseNoValueError,
        ParseLineError,
        ParseFileError,
	VMRunError,
	VMContextFetchNextError,
	VMContextFetchInvalidError
}


pub mod parser;
pub mod vm;
