// documentation
// https://gbdev.io/pandocs/CPU_Instruction_Set.html
// https://gbdev.io/gb-opcodes/optables/
// https://gbdev.io/gb-opcodes/optables/octal

use crate::{gameboy::Bus, sm83::Condition::NoCondition};

#[derive(Debug, Clone, Copy)]
enum RegisterName {
    B,
    C,
    D,
    E,
    H,
    L,
    A,
    AF,
    HL,
    BC,
    DE,
    SP,
    PC,
}

impl From<R8> for RegisterName {
    fn from(value: R8) -> Self {
        match value {
            R8::B => Self::B,
            R8::C => Self::C,
            R8::D => Self::D,
            R8::E => Self::E,
            R8::H => Self::H,
            R8::L => Self::L,
            R8::HL => Self::HL,
            R8::A => Self::A,
        }
    }
}

impl From<R16> for RegisterName {
    fn from(value: R16) -> Self {
        match value {
            R16::BC => RegisterName::BC,
            R16::DE => RegisterName::DE,
            R16::HL => RegisterName::HL,
            R16::SP => RegisterName::SP,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum R8 {
    B = 0,
    C = 1,
    D = 2,
    E = 3,
    H = 4,
    L = 5,
    HL = 6,
    A = 7,
}

impl From<u8> for R8 {
    fn from(value: u8) -> Self {
        match value & 0b111 {
            0 => R8::B,
            1 => R8::C,
            2 => R8::D,
            3 => R8::E,
            4 => R8::H,
            5 => R8::L,
            6 => R8::HL,
            7 => R8::A,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Condition {
    NZ = 0,
    Z = 1,
    NC = 2,
    C = 3,
    NoCondition,
}

impl From<u8> for Condition {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0 => Condition::NZ,
            1 => Condition::Z,
            2 => Condition::NC,
            3 => Condition::C,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum LoadArg {
    R8(R8),
    R16(R16),
    R16Mem(R16Mem),
    R16Stk(R16Stk),
    Condition(Condition), //condition
    Bit3(u8),
    Imm8,
    Imm16,
    Nothing,
}

#[derive(Debug, Clone, Copy)]
enum R16 {
    BC = 0,
    DE = 1,
    HL = 2,
    SP = 3,
}

impl From<u8> for R16 {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0 => Self::BC,
            1 => Self::DE,
            2 => Self::HL,
            3 => Self::SP,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum R16Stk {
    BC = 0,
    DE = 1,
    HL = 2,
    AF = 3,
}

impl From<R16Stk> for RegisterName {
    fn from(value: R16Stk) -> Self {
        match value {
            R16Stk::BC => Self::BC,
            R16Stk::DE => Self::DE,
            R16Stk::HL => Self::HL,
            R16Stk::AF => Self::AF,
        }
    }
}

impl From<u8> for R16Stk {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0 => Self::BC,
            1 => Self::DE,
            2 => Self::HL,
            3 => Self::AF,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum R16Mem {
    BC = 0,
    DE = 1,
    HLI = 2,
    HLD = 3,
}

impl From<R16Mem> for RegisterName {
    fn from(value: R16Mem) -> Self {
        match value {
            R16Mem::BC => Self::BC,
            R16Mem::DE => Self::DE,
            R16Mem::HLI => Self::HL,
            R16Mem::HLD => Self::HL,
        }
    }
}

impl From<u8> for R16Mem {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0 => Self::BC,
            1 => Self::DE,
            2 => Self::HLI,
            3 => Self::HLD,
            _ => unreachable!(),
        }
    }
}

enum RegisterCond {
    NZ = 0,
    Z = 1,
    NC = 2,
    C = 3,
}

enum Register {
    R8(R8),
    R16(R16),
    R16Mem(R16Mem),
    R16Stk(R16Stk),
    Immediate16,
    Immediate8,
}

#[derive(Debug, Clone, Copy)]
enum Flag {
    Zero = 7,
    Substraction = 6,
    HalfCarry = 5,
    Carry = 4,
}

trait HighLow {
    fn set_low(&mut self, value: u8);
    fn set_high(&mut self, value: u8);
    fn get_low(&self) -> u8;
    fn get_high(&self) -> u8;
}

impl HighLow for u16 {
    fn set_low(&mut self, value: u8) {
        *self &= 0xFF00;
        *self |= value as u16;
    }
    fn set_high(&mut self, value: u8) {
        *self &= 0x00FF;
        *self |= (value as u16) << 8;
    }
    fn get_low(&self) -> u8 {
        *self as u8
    }
    fn get_high(&self) -> u8 {
        (*self >> 8) as u8
    }
}

impl From<&LoadArg> for &str {
    fn from(value: &LoadArg) -> Self {
        match value {
            LoadArg::Bit3(_) => "",
            LoadArg::R8(r8) => match r8 {
                R8::B => "B",
                R8::C => "C",
                R8::D => "D",
                R8::E => "E",
                R8::H => "H",
                R8::L => "L",
                R8::HL => "[HL]",
                R8::A => "A",
            },
            LoadArg::R16Stk(r16stk) => match r16stk {
                R16Stk::BC => "BC",
                R16Stk::DE => "DE",
                R16Stk::HL => "HL",
                R16Stk::AF => "AF",
            },
            LoadArg::R16(r16) => match r16 {
                R16::BC => "BC",
                R16::DE => "DE",
                R16::HL => "HL",
                R16::SP => "SP",
            },
            LoadArg::R16Mem(r16_mem) => match r16_mem {
                R16Mem::BC => "[BC]",
                R16Mem::DE => "[DE]",
                R16Mem::HLI => "[HL+]",
                R16Mem::HLD => "[HL-]",
            },
            LoadArg::Condition(condition) => match condition {
                Condition::Z => "Z",
                Condition::NZ => "NZ",
                Condition::C => "C",
                Condition::NC => "NC",
                NoCondition => "",
            },
            LoadArg::Imm8 => "Imm8",
            LoadArg::Imm16 => "Imm16",
            LoadArg::Nothing => "",
        }
    }
}

impl std::fmt::Display for LoadArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.into())
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    operation: fn(&mut SM83, &mut Bus, LoadArg, LoadArg),
    name: &'static str,
    arg1: LoadArg,
    arg2: LoadArg,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.arg1, self.arg2) {
            (LoadArg::Nothing, LoadArg::Nothing) => f.write_str(self.name),
            (arg, LoadArg::Nothing) => f.write_fmt(format_args!("{} {}", self.name, arg)),
            (LoadArg::Condition(NoCondition), arg) => {
                f.write_fmt(format_args!("{} {}", self.name, arg))
            }
            (arg1, arg2) => f.write_fmt(format_args!("{} {}, {}", self.name, arg1, arg2)),
        }
    }
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            operation: SM83::illegal,
            name: "illegal",
            arg1: LoadArg::Nothing,
            arg2: LoadArg::Nothing,
        }
    }
}

pub use cpu_macro::uppercase;

macro_rules! op {
    ($instruction:ident, $arg1:expr, $arg2:expr) => {
        Instruction {
            operation: SM83::$instruction,
            name: uppercase!($instruction),
            arg1: $arg1,
            arg2: $arg2,
        }
    };
    ($instruction:ident, $arg1:expr) => {
        Instruction {
            operation: SM83::$instruction,
            name: uppercase!($instruction),
            arg1: $arg1,
            arg2: LoadArg::Nothing,
        }
    };
    ($instruction:ident) => {
        Instruction {
            operation: SM83::$instruction,
            name: uppercase!($instruction),
            arg1: LoadArg::Nothing,
            arg2: LoadArg::Nothing,
        }
    };
}

macro_rules! op_todo {
    ($name:literal) => {
        Instruction {
            operation: SM83::todo,
            name: $name,
            arg1: LoadArg::Nothing,
            arg2: LoadArg::Nothing,
        }
    };
}

#[derive(Debug)]
pub struct SM83 {
    pc: u16, //program counter
    sp: u16, //stack pointer
    af: u16, //accumalator and flags

    bc: u16,
    de: u16,

    hl: u16,   //HL Register
    ime: bool, //interrupt master enable

    operation_table: [Instruction; 256],
    operation_prefix_table: [Instruction; 256],
}

impl std::fmt::Display for SM83 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("pc:0x{:04X} sp:{:04X}", self.pc, self.sp))
    }
}

impl SM83 {
    fn illegal(&mut self, _bus: &mut Bus, _0: LoadArg, _1: LoadArg) {
        panic!("executing illegal instruction")
    }

    //https://gbdev.io/pandocs/Power_Up_Sequence.html?highlight=boot#cpu-registers
    pub fn new() -> Self {
        let mut cpu = SM83 {
            pc: 0x0100, //program counter
            sp: 0xFFFE, //stack counter
            af: 0,      //accumalator and flags

            bc: 0,
            de: 0,

            hl: 0,      //HL Register
            ime: false, //interrupt master enable

            operation_table: [Instruction::default(); 256],
            operation_prefix_table: [Instruction::default(); 256],
        };
        cpu.set_register(RegisterName::A, 0x11);
        cpu.set_register(RegisterName::D, 0xFF);
        cpu.set_register(RegisterName::H, 0x56);
        cpu.set_register(RegisterName::L, 0x0D);
        cpu.set_flag(Flag::Zero, true);
        cpu.build_table();
        cpu
    }

    fn get_instruction(opcode: u8) -> Instruction {
        let block = opcode >> 6;
        let bit_543 = opcode >> 3 & 0b111;
        let bit_210 = opcode & 0b111;
        match opcode {
            //block 0
            0b00000000 => op!(nop),
            0b00011000 => op!(jr, LoadArg::Condition(NoCondition), LoadArg::Imm8),
            0b00010000 => op_todo!("stop"),
            //block 1
            0b01110110 => Instruction {
                operation: |cpu, bus, _arg1, _arg2| Self::halt(cpu, bus),
                name: "HALT",
                arg1: LoadArg::Nothing,
                arg2: LoadArg::Nothing,
            },
            //block 3
            0b11101010 => op!(ld, LoadArg::Imm16, LoadArg::R8(R8::A)), //todo!("ld [imm16], a"),
            0b11111010 => op!(ld, LoadArg::R8(R8::A), LoadArg::Imm16), //todo!("ld a, [imm16]"),
            0b11111000 => Instruction {
                operation: |cpu, bus, _arg1, _arg2| Self::ld_hl_sp_imm8(cpu, bus),
                name: "LD HL, SP + Imm8",
                arg1: LoadArg::Nothing,
                arg2: LoadArg::Nothing,
            }, //todo!("ld hl, sp + imm8"),
            0b11111001 => op!(ld, LoadArg::R16(R16::SP), LoadArg::R16(R16::HL)), //todo!("ld sp, hl"),
            0b11001011 => op_todo!("prefix"),
            0b11100010 => op!(ldh, LoadArg::R8(R8::C), LoadArg::R8(R8::A)),
            0b11100000 => op!(ldh, LoadArg::Imm8, LoadArg::R8(R8::A)),
            0b11110010 => op!(ldh, LoadArg::R8(R8::A), LoadArg::R8(R8::C)),
            0b11110000 => op!(ldh, LoadArg::R8(R8::A), LoadArg::Imm8),
            0b11101000 => op_todo!("add sp, imm8"),
            0b11110011 => op!(di),
            0b11111011 => op!(ei),

            _ => {
                match block {
                    0b00 => {
                        let bit_54 = opcode >> 4 & 0b11;
                        let bit_3210 = opcode & 0b1111;
                        match bit_3210 {
                            0b0001 => return op!(ld, LoadArg::R16(bit_54.into()), LoadArg::Imm16),
                            0b0010 => {
                                return op!(ld, LoadArg::R16Mem(bit_54.into()), LoadArg::R8(R8::A));
                            }
                            0b1010 => {
                                return op!(ld, LoadArg::R8(R8::A), LoadArg::R16Mem(bit_54.into()));
                            }
                            0b1000 => return op!(ld, LoadArg::Imm16, LoadArg::R16(R16::SP)),
                            0b0011 => return op!(inc, LoadArg::R16(bit_54.into())),
                            0b1011 => return op!(dec, LoadArg::R16(bit_54.into())),
                            0b1001 => {
                                return op!(add, LoadArg::R16(R16::HL), LoadArg::R16(bit_54.into()));
                            }
                            _ => (),
                        };

                        let bit_210 = opcode & 0b111;
                        match bit_210 {
                            0b100 => return op!(inc, LoadArg::R8(bit_543.into())),
                            0b101 => return op!(dec, LoadArg::R8(bit_543.into())),
                            0b110 => return op!(ld, LoadArg::R8(bit_543.into()), LoadArg::Imm8),
                            0b111 => match bit_543 & 0b111 {
                                0b000 => return op!(rlca),
                                0b001 => return op!(rrca),
                                0b010 => return op!(rla),
                                0b011 => return op!(rra),
                                0b100 => return op!(daa),
                                0b101 => return op!(cpl),
                                0b110 => return op!(scf),
                                0b111 => return op!(ccf),
                                _ => unreachable!(),
                            },
                            0b000 => {
                                if opcode & 0b11100111 == 0b00100000 {
                                    let bit_43 = opcode >> 3 & 0b11;
                                    return op!(
                                        jr,
                                        LoadArg::Condition(bit_43.into()),
                                        LoadArg::Imm8
                                    );
                                }
                            }
                            _ => (),
                        };

                        todo!()
                    }
                    0b01 => {
                        let r_dest = bit_543.into();
                        let r_src = bit_210.into();
                        if let R8::HL = r_dest
                            && let R8::HL = r_src
                        {
                            // op!(halt)
                            Instruction {
                                operation: |cpu, bus, _arg1, _arg2| Self::halt(cpu, bus),
                                name: "HALT",
                                arg1: LoadArg::Nothing,
                                arg2: LoadArg::Nothing,
                            }
                        } else {
                            op!(ld, LoadArg::R8(r_dest), LoadArg::R8(r_src))
                        }
                    }
                    0b10 => {
                        let operation_type = opcode >> 3 & 0b111;
                        let r = LoadArg::R8(opcode.into());
                        match operation_type {
                            0b000 => op!(add, r, LoadArg::Nothing), //add r8
                            0b001 => op!(adc, r, LoadArg::Nothing), //adc r8
                            0b010 => op!(sub, r, LoadArg::Nothing), //sub r8
                            0b011 => op!(sbc, r, LoadArg::Nothing), //sbc r8
                            0b100 => op!(and, r, LoadArg::Nothing), //and r8
                            0b101 => op!(xor, r, LoadArg::Nothing), //xor r8
                            0b110 => op!(or, r, LoadArg::Nothing),  //or r8
                            0b111 => op!(cp, r, LoadArg::Nothing),  //cp r8
                            _ => unreachable!("operation_type is three bit"),
                        }
                    }
                    0b11 => {
                        let bit_765 = (opcode & 0b11100000) >> 5;
                        let bit_543 = (opcode & 0b00111000) >> 3;
                        let bit_210 = opcode & 0b00000111;
                        if bit_210 == 0b110 {
                            let arithmetic_type = opcode >> 3 & 0b111;
                            //arithetic imm8
                            return match arithmetic_type {
                                0b000 => op!(add, LoadArg::Imm8), //add imm8
                                0b001 => op!(adc, LoadArg::Imm8), //adc imm8
                                0b010 => op!(sub, LoadArg::Imm8), //sub imm8
                                0b011 => op!(sbc, LoadArg::Imm8), //sbc imm8
                                0b100 => op!(and, LoadArg::Imm8), //and imm8
                                0b101 => op!(xor, LoadArg::Imm8), //xor imm8
                                0b110 => op!(or, LoadArg::Imm8),  //or imm8
                                0b111 => op!(cp, LoadArg::Imm8),  //cp imm8
                                _ => unreachable!("arithmetic_type is only 3bit"),
                            };
                        }
                        let bit_43 = opcode >> 3 & 0b11;
                        match (bit_765, bit_43, bit_210) {
                            (0b110, 0b01, 0b001) => {
                                return op!(
                                    ret,
                                    LoadArg::Condition(Condition::NoCondition),
                                    LoadArg::Nothing
                                );
                            } //todo!("ret"),
                            (0b110, 0b11, 0b001) => return op!(reti),
                            (0b110, cond, 0b000) => {
                                return op!(ret, LoadArg::Condition(cond.into()), LoadArg::Nothing);
                            } // todo!("ret cond"),
                            (0b110, cond, 0b010) => {
                                return op!(jp, LoadArg::Condition(cond.into()), LoadArg::Imm16);
                            } //todo!("jp cond, imm16"),
                            (0b110, 0b00, 0b011) => {
                                return op!(
                                    jp,
                                    LoadArg::Condition(Condition::NoCondition),
                                    LoadArg::Imm16
                                );
                            } //todo!("jp imm16"),
                            (0b111, 0b01, 0b001) => {
                                return op!(
                                    jp,
                                    LoadArg::Condition(Condition::NoCondition),
                                    LoadArg::R16(R16::HL)
                                );
                            } //todo!("jp hl"),
                            (0b110, cond, 0b100) => {
                                return op!(call, LoadArg::Condition(cond.into()), LoadArg::Imm16);
                            } // call cond, imm16
                            (0b110, 0b01, 0b101) => {
                                return op!(call, LoadArg::Condition(NoCondition), LoadArg::Imm16);
                            } //call imm16
                            (_, _, _) => (),
                        }
                        if opcode & 0b11000111 == 0b11000111 {
                            return op!(rst, LoadArg::Bit3(bit_543));
                        }

                        let bit_3210 = opcode & 0b1111;
                        let bit_54 = opcode >> 4 & 0b11;
                        match bit_3210 {
                            0b0001 => return op!(pop, LoadArg::R16Stk(bit_54.into())),
                            0b0101 => return op!(push, LoadArg::R16Stk(bit_54.into())),
                            _ => return op_todo!("illegal"),
                        };
                    }
                    _ => op_todo!("illegal"),
                }
            }
        }
    }

    fn get_prefix_instruction(opcode: u8) -> Instruction {
        let bit_210 = opcode & 0b111;
        let bit_543 = (opcode >> 3) & 0b111;
        let bit_76 = (opcode >> 6);
        let r8 = LoadArg::R8(bit_210.into());
        let b3 = LoadArg::Bit3(bit_543);
        match bit_76 {
            0b00 => match bit_543 {
                0b000 => op!(rlc, r8),
                0b001 => op!(rrc, r8),
                0b010 => op!(rl, r8),
                0b011 => op!(rr, r8),
                0b100 => op!(sla, r8),
                0b101 => op!(sra, r8),
                0b110 => op!(swap, r8),
                0b111 => op!(srl, r8),
                _ => unreachable!(),
            },
            0b01 => op!(bit, b3, r8),
            0b10 => op!(res, b3, r8),
            0b11 => op!(set, b3, r8),
            _ => unreachable!(),
        }
    }

    fn build_table(&mut self) {
        for i in 0..256 {
            self.operation_table[i] = Self::get_instruction(i as u8);
            self.operation_prefix_table[i] = Self::get_prefix_instruction(i as u8)
        }
    }

    fn get_register(&self, r: RegisterName) -> u16 {
        match r {
            RegisterName::B => self.bc.get_high() as u16,
            RegisterName::C => self.bc.get_low() as u16,
            RegisterName::D => self.de.get_high() as u16,
            RegisterName::E => self.de.get_low() as u16,
            RegisterName::H => self.hl.get_high() as u16,
            RegisterName::L => self.hl.get_low() as u16,
            RegisterName::A => self.af.get_high() as u16,
            RegisterName::HL => self.hl,
            RegisterName::BC => self.bc,
            RegisterName::DE => self.de,
            RegisterName::SP => self.sp,
            RegisterName::PC => self.pc,
            RegisterName::AF => self.af,
        }
    }

    fn set_register(&mut self, r: RegisterName, value: u16) {
        match r {
            RegisterName::B => self.bc.set_high(value as u8),
            RegisterName::C => self.bc.set_low(value as u8),
            RegisterName::D => self.de.set_high(value as u8),
            RegisterName::E => self.de.set_low(value as u8),
            RegisterName::H => self.hl.set_high(value as u8),
            RegisterName::L => self.hl.set_low(value as u8),
            RegisterName::A => self.af.set_high(value as u8),
            RegisterName::HL => self.hl = value,
            RegisterName::BC => self.bc = value,
            RegisterName::DE => self.de = value,
            RegisterName::SP => self.sp = value,
            RegisterName::PC => self.pc = value,
            RegisterName::AF => self.af = value,
        }
    }

    fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.af |= (value as u16) << (flag as u8);
        } else {
            self.af &= !(1 << (flag as u16));
        }
    }

    //set flags: z = zero, n = substraction, h = half_carry, c = carry
    fn set_flags(&mut self, z: bool, n: bool, h: bool, c: bool) {
        self.set_flag(Flag::Zero, z);
        self.set_flag(Flag::Substraction, n);
        self.set_flag(Flag::HalfCarry, h);
        self.set_flag(Flag::Carry, c);
    }

    fn get_flag(&mut self, flag: Flag) -> bool {
        (self.af & (1 << (flag as u8))) != 0
    }

    fn nop(&mut self, _bus: &mut Bus, _0: LoadArg, _1: LoadArg) {}

    fn inc(&mut self, bus: &mut Bus, register: LoadArg, _1: LoadArg) {
        match register {
            LoadArg::R8(r8) => {
                let original = self.get_r8(bus, r8);
                let modified = original.wrapping_add(1);
                self.set_r8(bus, r8, modified);

                self.set_flag(Flag::Zero, modified == 0);
                self.set_flag(Flag::Substraction, false);
                self.set_flag(Flag::HalfCarry, original & 0xF < modified & 0xF);
            }
            LoadArg::R16(r16) => {
                self.set_register(r16.into(), self.get_register(r16.into()).wrapping_add(1))
            }
            _ => unreachable!(),
        }
    }

    fn dec(&mut self, bus: &mut Bus, register: LoadArg, _1: LoadArg) {
        match register {
            LoadArg::R8(r8) => {
                let original = self.get_r8(bus, r8);
                let modified = original.wrapping_sub(1);
                self.set_r8(bus, r8, modified);

                self.set_flag(Flag::Zero, modified == 0);
                self.set_flag(Flag::Substraction, true);
                self.set_flag(Flag::HalfCarry, original & 0xF > modified & 0xF);
            }
            LoadArg::R16(r16) => {
                self.set_register(r16.into(), self.get_register(r16.into()).wrapping_sub(1))
            }
            _ => unreachable!(),
        }
    }

    fn imm8(&mut self, bus: &mut Bus) -> u8 {
        self.fetch(bus)
    }

    fn imm16(&mut self, bus: &mut Bus) -> u16 {
        let byte1 = self.fetch(bus) as u16;
        let byte2 = self.fetch(bus) as u16;
        (byte2 << 8) | byte1
    }

    fn get_r8(&mut self, bus: &mut Bus, r8: R8) -> u8 {
        match r8 {
            R8::HL => bus.read(self.get_register(RegisterName::HL)),
            x => self.get_register(x.into()) as u8,
        }
    }

    fn set_r8(&mut self, bus: &mut Bus, r8: R8, value: u8) {
        match r8 {
            R8::HL => bus.write(self.hl, value),
            r8 => self.set_register(r8.into(), value as u16),
        }
    }

    fn ld_hl_sp_imm8(&mut self, bus: &mut Bus) {
        let sp = self.sp;
        let signed_imm8 = self.imm8(bus) as i8;
        let offset = signed_imm8 as i16 as u16; //imm8 is signed
        let low = (signed_imm8 as u8) as u16;
        self.hl = sp.wrapping_add(offset);
        self.set_flags(
            false,
            false,
            (sp & 0x000F) + (low & 0x000F) > 0x000F,
            (sp & 0x00FF) + (low & 0x00FF) > 0x00FF,
        );
    }

    fn todo(&mut self, _bus: &mut Bus, _0: LoadArg, _1: LoadArg) {
        todo!("op_todo")
    }

    fn halt(&mut self, _bus: &mut Bus) {
        todo!("halt")
    }

    fn ld(&mut self, bus: &mut Bus, dst: LoadArg, src: LoadArg) {
        let src_byte: u16 = match src {
            LoadArg::R8(register_r8) => self.get_r8(bus, register_r8) as u16,
            LoadArg::R16(register_r16) => self.get_register(register_r16.into()),
            LoadArg::R16Mem(register_r16_mem) => {
                let addr = self.get_register(register_r16_mem.into());
                self.hl = match register_r16_mem {
                    R16Mem::HLI => self.hl.wrapping_add(1),
                    R16Mem::HLD => self.hl.wrapping_sub(1),
                    _ => self.hl,
                };
                bus.read(addr) as u16
            }
            LoadArg::Imm8 => self.imm8(bus) as u16,
            LoadArg::Imm16 => self.imm16(bus),
            _ => unreachable!(),
        };

        match dst {
            LoadArg::R8(register_r8) => self.set_register(register_r8.into(), src_byte),
            LoadArg::R16(register_r16) => self.set_register(register_r16.into(), src_byte),
            LoadArg::R16Mem(register_r16_mem) => {
                let addr = self.get_register(register_r16_mem.into());
                self.hl = match register_r16_mem {
                    R16Mem::HLI => self.hl.wrapping_add(1),
                    R16Mem::HLD => self.hl.wrapping_sub(1),
                    _ => self.hl,
                };
                bus.write(addr, src_byte as u8);
            }
            LoadArg::Imm8 => unreachable!("dst can't be a Imm8"),
            LoadArg::Imm16 => {
                let addr = self.imm16(bus);
                bus.write(addr, (src_byte & 0xFF) as u8);
                if let LoadArg::R16(_) = src {
                    //used of LD [imm16],SP
                    bus.write(addr.wrapping_add(1), (src_byte >> 8) as u8);
                }
            }
            _ => unreachable!(),
        };
    }

    fn ldh(&mut self, bus: &mut Bus, dst: LoadArg, src: LoadArg) {
        match (dst, src) {
            (LoadArg::R8(R8::C), LoadArg::R8(R8::A)) => {
                let a = self.get_register(RegisterName::A);
                let c = self.get_register(RegisterName::C);
                bus.write(0xFF00 | c, a as u8);
            }
            (LoadArg::Imm8, LoadArg::R8(R8::A)) => {
                let a = self.get_register(RegisterName::A);
                let imm8 = self.imm8(bus) as u16;
                bus.write(0xFF00 | imm8, a as u8);
            }
            (LoadArg::R8(R8::A), LoadArg::R8(R8::C)) => {
                let c = self.get_register(RegisterName::C);
                let value = bus.read(0xFF00 | c);
                self.set_register(RegisterName::A, value as u16);
            }
            (LoadArg::R8(R8::A), LoadArg::Imm8) => {
                let imm8 = self.imm8(bus) as u16;
                let value = bus.read(0xFF00 | imm8);
                self.set_register(RegisterName::A, value as u16);
            }
            _ => unreachable!(),
        }
    }

    //shift

    fn rotate(&mut self, left: bool, use_carry: bool, value: u8) -> u8 {
        let leftmost = value & 0b10000000;
        let rightmost = value & 0b1;

        let moving_bit = if use_carry {
            self.get_flag(Flag::Carry) as u8
        } else {
            if left { leftmost } else { rightmost }
        };

        let output = if left {
            self.set_flag(Flag::Carry, leftmost != 0);
            (value << 1) | (moving_bit)
        } else {
            self.set_flag(Flag::Carry, rightmost != 0);
            (value >> 1) | (moving_bit << 7)
        };

        self.set_flag(Flag::Zero, output == 0);
        self.set_flag(Flag::Substraction, false);
        self.set_flag(Flag::HalfCarry, false);

        //println!("rotate {left:?} {use_carry:?} value: {value:#b} output: {output:#b}");
        output
    }

    fn rlca(&mut self, bus: &mut Bus, _0: LoadArg, _1: LoadArg) {
        todo!()
    }

    fn rrca(&mut self, bus: &mut Bus, _0: LoadArg, _1: LoadArg) {
        let a = self.get_register(RegisterName::A) as u8;
        let a = self.rotate(false, false, a);
        self.set_register(RegisterName::A, a as u16);
    }

    fn rla(&mut self, bus: &mut Bus, _0: LoadArg, _1: LoadArg) {
        let a = self.get_register(RegisterName::A) as u8;
        let a = self.rotate(true, true, a);
        self.set_register(RegisterName::A, a as u16);
    }

    fn rra(&mut self, bus: &mut Bus, _0: LoadArg, _1: LoadArg) {
        todo!()
    }

    fn daa(&mut self, bus: &mut Bus, _0: LoadArg, _1: LoadArg) {
        todo!()
    }
    fn cpl(&mut self, bus: &mut Bus, _0: LoadArg, _1: LoadArg) {
        todo!()
    }
    fn scf(&mut self, bus: &mut Bus, _0: LoadArg, _1: LoadArg) {
        self.set_flag(Flag::Zero, false);
        self.set_flag(Flag::HalfCarry, false);
        self.set_flag(Flag::Carry, true);
    }

    fn ccf(&mut self, _bus: &mut Bus, _0: LoadArg, _1: LoadArg) {
        self.set_flag(Flag::Zero, false);
        self.set_flag(Flag::HalfCarry, false);
        let c = self.get_flag(Flag::Carry);
        self.set_flag(Flag::Carry, !c);
    }

    /*Prefix Instruction */

    fn rlc(&mut self, _bus: &mut Bus, register: LoadArg, _1: LoadArg) {
        todo!()
    }

    fn rrc(&mut self, _bus: &mut Bus, register: LoadArg, _1: LoadArg) {
        todo!()
    }

    fn rl(&mut self, bus: &mut Bus, register: LoadArg, _1: LoadArg) {
        match register {
            LoadArg::R8(r8) => {
                let value = self.get_r8(bus, r8);
                self.rotate(true, true, value);
                self.set_r8(bus, r8, value);
            }
            _ => unreachable!(),
        }
    }

    fn rr(&mut self, _bus: &mut Bus, register: LoadArg, _1: LoadArg) {
        todo!()
    }

    fn sla(&mut self, _bus: &mut Bus, register: LoadArg, _1: LoadArg) {
        todo!()
    }

    fn sra(&mut self, _bus: &mut Bus, register: LoadArg, _1: LoadArg) {
        todo!()
    }

    fn swap(&mut self, _bus: &mut Bus, register: LoadArg, _1: LoadArg) {
        todo!()
    }

    fn srl(&mut self, _bus: &mut Bus, register: LoadArg, _1: LoadArg) {
        todo!()
    }

    fn bit(&mut self, bus: &mut Bus, bits: LoadArg, register: LoadArg) {
        let bits = match bits {
            LoadArg::Bit3(value) => value,
            _ => unreachable!(),
        };

        let value = match register {
            LoadArg::R8(r8) => self.get_r8(bus, r8),
            _ => unreachable!(),
        };

        self.set_flag(Flag::Zero, (value & (1 << bits)) == 0);
        self.set_flag(Flag::Substraction, false);
        self.set_flag(Flag::HalfCarry, true);
    }

    fn set(&mut self, bus: &mut Bus, bits: LoadArg, register: LoadArg) {
        let bits = match bits {
            LoadArg::Bit3(value) => value,
            _ => unreachable!(),
        };

        match register {
            LoadArg::R8(r8) => {
                let mut value = self.get_r8(bus, r8);
                value = value | (1 << bits);
                self.set_r8(bus, r8, value);
            }
            _ => unreachable!(),
        };
    }

    fn res(&mut self, bus: &mut Bus, bits: LoadArg, register: LoadArg) {
        let bits = match bits {
            LoadArg::Bit3(value) => value,
            _ => unreachable!(),
        };

        match register {
            LoadArg::R8(r8) => {
                let mut value = self.get_r8(bus, r8);
                value = value & !(1 << bits);
                self.set_r8(bus, r8, value);
            }
            _ => unreachable!(),
        };
    }
    /*Arimetic 8bit */

    fn add8(&mut self, value: u8, carry: bool) {
        let a = self.get_register(RegisterName::A) as u8;
        let c = carry as u8;
        let output = a.wrapping_add(value).wrapping_add(carry as u8);

        self.set_flag(Flag::Zero, output == 0);
        self.set_flag(Flag::Substraction, false);
        self.set_flag(
            Flag::HalfCarry,
            (a as u16 & 0xF) + (value as u16 & 0xF) + (c as u16 & 0xF) > 0xF,
        );
        self.set_flag(
            Flag::Carry,
            (a as u16 & 0xFF) + (value as u16 & 0xFF) + (c as u16 & 0xFF) > 0x00FF,
        );
        self.set_register(RegisterName::A, output as u16);
    }

    fn add16(&mut self, dst: u16, src: u16) -> u16 {
        let output = dst.wrapping_add(src);

        self.set_flag(Flag::Zero, output == 0);
        self.set_flag(Flag::Substraction, false);
        self.set_flag(
            Flag::HalfCarry,
            (src as u32 & 0xFFF) + (dst as u32 & 0xFFF) > 0xFFF,
        );
        self.set_flag(
            Flag::Carry,
            (src as u32 & 0xFFFF) + (dst as u32 & 0xFFFF) > 0xFFFF,
        );
        output
    }

    fn add(&mut self, bus: &mut Bus, dst: LoadArg, src: LoadArg) {
        match (dst, src) {
            (LoadArg::R8(R8::A), src) => {
                let value = match src {
                    LoadArg::R8(r8) => self.get_r8(bus, r8),
                    LoadArg::Imm8 => self.imm8(bus),
                    _ => unreachable!(),
                };

                self.add8(value, false);
            }
            (LoadArg::R16(R16::HL), LoadArg::R16(src)) => {
                let dst_byte = self.get_register(RegisterName::HL);
                let src_byte = self.get_register(src.into());
                let result = self.add16(dst_byte, src_byte);
                self.set_register(RegisterName::HL, result);
            }
            (LoadArg::R16(R16::SP), LoadArg::Imm8) => {
                let imm8 = self.imm8(bus) as i8 as i16;
                let src = self.sp;
                self.sp = self.sp.wrapping_add_signed(imm8 as i16);

                self.set_flags(
                    false,
                    false,
                    (src as u16 & 0xF) + (imm8 as u16 & 0xF) > 0xF,
                    (src as u16 & 0xFF) + (imm8 as u16 & 0xFF) > 0xFF,
                );
            }
            _ => unreachable!(),
        }
    }

    fn adc(&mut self, bus: &mut Bus, param: LoadArg, _0: LoadArg) {
        let value = match param {
            LoadArg::R8(r8) => self.get_r8(bus, r8),
            LoadArg::Imm8 => self.imm8(bus),
            _ => unreachable!(),
        };
        let carry = self.get_flag(Flag::Carry);
        self.add8(value, carry);
    }

    fn sub8(&mut self, value: u8, carry: bool) {
        let a = self.get_register(RegisterName::A) as u8;
        let c = carry as u8;
        let output = a.wrapping_sub(value.wrapping_add(c));

        self.set_flags(
            output == 0,
            true,
            ((value as u16 & 0xF) + (c as u16 & 0xF)) > (a as u16 & 0xF),
            ((value as u16 & 0xFF) + (c as u16 & 0xFF)) > (a as u16 & 0xFF),
        );
        self.set_register(RegisterName::A, output as u16);
    }

    fn sub(&mut self, bus: &mut Bus, param: LoadArg, _0: LoadArg) {
        let value = match param {
            LoadArg::R8(r8) => self.get_r8(bus, r8),
            LoadArg::Imm8 => self.imm8(bus),
            _ => unreachable!(),
        };
        self.sub8(value, false);
    }

    fn sbc(&mut self, bus: &mut Bus, param: LoadArg, _0: LoadArg) {
        let value = match param {
            LoadArg::R8(r8) => self.get_r8(bus, r8),
            LoadArg::Imm8 => self.imm8(bus),
            _ => unreachable!(),
        };
        let carry = self.get_flag(Flag::Carry);
        self.sub8(value, carry);
    }

    fn and(&mut self, bus: &mut Bus, param: LoadArg, _0: LoadArg) {
        let value = match param {
            LoadArg::R8(r8) => self.get_r8(bus, r8) as u16,
            LoadArg::Imm8 => self.imm8(bus) as u16,
            _ => unreachable!(),
        };
        let mut a = self.get_register(RegisterName::A);
        a &= value;
        self.set_register(RegisterName::A, a);
        self.set_flags(a == 0, false, true, false);
    }

    fn xor(&mut self, bus: &mut Bus, param: LoadArg, _0: LoadArg) {
        let value = match param {
            LoadArg::R8(r8) => self.get_r8(bus, r8) as u16,
            LoadArg::Imm8 => self.imm8(bus) as u16,
            _ => unreachable!(),
        };

        let mut a = self.get_register(RegisterName::A);
        a ^= value;
        self.set_register(RegisterName::A, a);
        self.set_flags(a == 0, false, false, false);
    }

    fn or(&mut self, bus: &mut Bus, param: LoadArg, _0: LoadArg) {
        let value = match param {
            LoadArg::R8(r8) => self.get_r8(bus, r8) as u16,
            LoadArg::Imm8 => self.imm8(bus) as u16,
            _ => unreachable!(),
        };

        let mut a = self.get_register(RegisterName::A);
        a |= value;
        self.set_register(RegisterName::A, a);
        self.set_flags(a == 0, false, false, false);
    }

    fn cp(&mut self, bus: &mut Bus, param: LoadArg, _0: LoadArg) {
        let value = match param {
            LoadArg::R8(r8) => self.get_r8(bus, r8),
            LoadArg::Imm8 => self.imm8(bus),
            _ => unreachable!(),
        };

        let a_original = self.get_register(RegisterName::A);
        self.sub8(value, false);
        //reset a to it's original value because compare only change the flags
        self.set_register(RegisterName::A, a_original);
    }

    fn is_cond(&mut self, cond: Condition) -> bool {
        match cond {
            Condition::Z => self.get_flag(Flag::Zero),
            Condition::NZ => !self.get_flag(Flag::Zero),
            Condition::C => self.get_flag(Flag::Carry),
            Condition::NC => !self.get_flag(Flag::Carry),
            Condition::NoCondition => true,
        }
    }

    fn jp(&mut self, bus: &mut Bus, cond: LoadArg, dest: LoadArg) {
        let can_jump = match cond {
            LoadArg::Condition(cond) => self.is_cond(cond),
            _ => unreachable!(),
        };

        let jump_addr = match dest {
            LoadArg::Imm16 => self.imm16(bus),
            LoadArg::R16(R16::HL) => self.get_register(RegisterName::HL),
            _ => unreachable!(),
        };

        if !can_jump {
            return;
        }

        self.set_register(RegisterName::PC, jump_addr);
    }

    fn jr(&mut self, bus: &mut Bus, cond: LoadArg, dest: LoadArg) {
        let can_jump = match cond {
            LoadArg::Condition(cond) => self.is_cond(cond),
            _ => unreachable!(),
        };

        let jump_addr = match dest {
            LoadArg::Imm8 => self.imm8(bus) as i8,
            _ => unreachable!(),
        };

        if !can_jump {
            return;
        }

        self.pc = self.pc.wrapping_add_signed(jump_addr as i16);
    }

    fn stack_push16(&mut self, bus: &mut Bus, data: u16) {
        let low = (data & 0xFF) as u8;
        let high = ((data >> 8) & 0xFF) as u8;
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, high);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, low);
    }

    fn stack_pop16(&mut self, bus: &mut Bus) -> u16 {
        let low = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        let high = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        let out = ((high as u16) << 8) | (low as u16);
        out
    }

    fn push(&mut self, bus: &mut Bus, reg: LoadArg, _1: LoadArg) {
        let value = match reg {
            LoadArg::R16Stk(r16stk) => self.get_register(r16stk.into()),
            _ => unreachable!(),
        };

        self.stack_push16(bus, value);
    }

    fn pop(&mut self, bus: &mut Bus, reg: LoadArg, _1: LoadArg) {
        let value = self.stack_pop16(bus);

        match reg {
            LoadArg::R16Stk(r16stk) => self.set_register(r16stk.into(), value),
            _ => unreachable!(),
        };
    }

    fn call(&mut self, bus: &mut Bus, cond: LoadArg, addr: LoadArg) {
        let can_call = match cond {
            LoadArg::Condition(cond) => self.is_cond(cond),
            _ => unreachable!(),
        };

        let jump_addr = match addr {
            LoadArg::Imm16 => self.imm16(bus),
            _ => unreachable!(),
        };

        if !can_call {
            return;
        }
        self.stack_push16(bus, self.pc);
        self.pc = jump_addr;
    }

    fn rst(&mut self, bus: &mut Bus, vec: LoadArg, _1: LoadArg) {
        let addr = if let LoadArg::Bit3(value) = vec {
            value * 8
        } else {
            unreachable!()
        };

        self.stack_push16(bus, self.pc.wrapping_add(1));
        self.pc = addr as u16;
    }

    fn ret(&mut self, bus: &mut Bus, cond: LoadArg, _0: LoadArg) {
        let can_ret = match cond {
            LoadArg::Condition(cond) => self.is_cond(cond),
            _ => unreachable!(),
        };

        if !can_ret {
            return;
        }
        let addr = self.stack_pop16(bus);
        self.pc = addr;
    }

    fn ei(&mut self, _bus: &mut Bus, _0: LoadArg, _1: LoadArg) {
        self.ime = true
    }

    fn di(&mut self, _bus: &mut Bus, _0: LoadArg, _1: LoadArg) {
        self.ime = false
    }

    fn reti(&mut self, bus: &mut Bus, _0: LoadArg, _1: LoadArg) {
        self.ei(bus, LoadArg::Nothing, LoadArg::Nothing);
        self.ret(bus, LoadArg::Condition(NoCondition), LoadArg::Nothing);
    }

    fn peek8(&mut self, bus: &mut Bus) -> u8 {
        let byte = bus.read(self.pc);
        byte
    }

    fn peek16(&mut self, bus: &mut Bus) -> u16 {
        let low = bus.read(self.pc) as u16;
        let high = bus.read(self.pc + 1) as u16;
        high << 8 | low
    }

    fn fetch(&mut self, bus: &mut Bus) -> u8 {
        let byte = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    fn debug_arg(&mut self, bus: &mut Bus, arg: LoadArg) -> String {
        match arg {
            LoadArg::Bit3(bit3) => {
                format!("{bit3}")
            }
            LoadArg::R8(r8) => {
                let value = self.get_r8(bus, r8);
                format!("0x{value:02X}")
            }
            LoadArg::R16(r16) => {
                let value = self.get_register(r16.into());
                format!("0x{value:04X}")
            }
            LoadArg::R16Mem(r16_mem) => {
                let reg = match r16_mem {
                    R16Mem::BC => self.bc,
                    R16Mem::DE => self.de,
                    R16Mem::HLI => self.hl,
                    R16Mem::HLD => self.hl,
                };
                let value = bus.read(reg);
                format!("0x{reg:04X}:0x{value:04X}")
            }
            LoadArg::R16Stk(r16_stk) => {
                let value = self.get_register(r16_stk.into());
                format!("0x{value:04X}")
            }
            LoadArg::Condition(condition) => {
                let bool = self.is_cond(condition);
                format!("{bool}")
            }
            LoadArg::Imm8 => {
                let imm8 = self.peek8(bus);
                format!("0x{imm8:02X}")
            }
            LoadArg::Imm16 => {
                let imm16 = self.peek16(bus);
                format!("0x{imm16:04X}")
            }
            LoadArg::Nothing => "".to_string(),
        }
    }

    fn debug_op(&mut self, bus: &mut Bus, ins: &Instruction) {
        let pc = self.pc.wrapping_sub(1);
        let name = ins.name;
        match (ins.arg1, ins.arg2) {
            (LoadArg::Nothing, LoadArg::Nothing) => {
                println!("0x{pc:04X}: {name}")
            }
            (arg1, LoadArg::Nothing) => {
                let v1 = self.debug_arg(bus, arg1);
                println!("0x{pc:04X}: {name} {arg1}[{v1}]")
            }
            (arg1, arg2) => {
                let v1 = self.debug_arg(bus, arg1);
                let v2 = self.debug_arg(bus, arg2);
                println!("0x{pc:04X}: {name} {arg1}[{v1}] {arg2}[{v2}]")
            }
        }
    }

    pub fn step(&mut self, bus: &mut Bus) {
        let opcode = self.fetch(bus);
        let op = if opcode != 0xCB {
            self.operation_table[opcode as usize]
        } else {
            let opcode = self.fetch(bus);
            self.operation_prefix_table[opcode as usize]
        };

        self.debug_op(bus, &op);
        (op.operation)(self, bus, op.arg1, op.arg2);
    }
}

#[cfg(test)]
mod tests {

    use crate::mbc::{mbc1::MBC1, rom_only::RomOnly};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_flag() {
        let mut cpu = SM83::new();

        cpu.af = 0;

        cpu.set_flag(Flag::Zero, true);
        assert!(cpu.get_flag(Flag::Zero) == true);

        cpu.set_flag(Flag::Zero, false);
        assert!(cpu.get_flag(Flag::Zero) == false);

        cpu.set_flags(true, false, true, false);
        assert!(cpu.get_flag(Flag::Zero) == true);
        assert!(cpu.get_flag(Flag::Substraction) == false);
        assert!(cpu.get_flag(Flag::HalfCarry) == true);
        assert!(cpu.get_flag(Flag::Carry) == false);

        cpu.set_flags(false, true, false, true);
        assert!(cpu.get_flag(Flag::Zero) == false);
        assert!(cpu.get_flag(Flag::Substraction) == true);
        assert!(cpu.get_flag(Flag::HalfCarry) == false);
        assert!(cpu.get_flag(Flag::Carry) == true);
    }
}
