// documentation
// https://gbdev.io/pandocs/CPU_Instruction_Set.html
// https://gbdev.io/gb-opcodes/optables/
// https://gbdev.io/gb-opcodes/optables/octal

use crate::gameboy::Bus;

pub struct SM83 {
    pc: u16, //program counter
    sp: u16, //stack pointer
    af: u16, //accumalator and flags

    bc: u16,
    de: u16,

    hl: u16, //HL Register
    ir: u8,  //instruction register
    ie: u8,  //interrupt enable
}

enum RegisterName {
    B,
    C,
    D,
    E,
    H,
    L,
    A,
    HL,
    BC,
    DE,
    SP,
    PC,
}

impl From<RegisterR8> for RegisterName {
    fn from(value: RegisterR8) -> Self {
        match value {
            RegisterR8::B => Self::B,
            RegisterR8::C => Self::C,
            RegisterR8::D => Self::D,
            RegisterR8::E => Self::E,
            RegisterR8::H => Self::H,
            RegisterR8::L => Self::L,
            RegisterR8::HL => Self::HL,
            RegisterR8::A => Self::A,
        }
    }
}

enum RegisterR8 {
    B = 0,
    C = 1,
    D = 2,
    E = 3,
    H = 4,
    L = 5,
    HL = 6,
    A = 7,
}

impl TryFrom<u8> for RegisterR8 {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => RegisterR8::B,
            1 => RegisterR8::C,
            2 => RegisterR8::D,
            3 => RegisterR8::E,
            4 => RegisterR8::H,
            5 => RegisterR8::L,
            6 => RegisterR8::HL,
            7 => RegisterR8::A,
            _ => return Err(value),
        })
    }
}

enum RegisterR16 {
    BC = 0,
    DE = 1,
    HL = 2,
    SP = 3,
}

enum RegisterR16Stk {
    BC = 0,
    DE = 1,
    HL = 2,
    AF = 3,
}
enum RegisterR16Mem {
    BC = 0,
    DE = 1,
    HLPlus = 2,
    HLMinus = 3,
}

enum RegisterCond {
    NZ = 0,
    Z = 1,
    NC = 2,
    C = 3,
}

enum Register {
    R8(RegisterR8),
    R16(RegisterR16),
    R16Mem(RegisterR16Mem),
    R16Stk(RegisterR16Stk),
    Immediate16,
    Immediate8,
}

enum RegisterFlag {
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
impl SM83 {
    //https://gbdev.io/pandocs/Power_Up_Sequence.html?highlight=boot#cpu-registers
    pub fn new() -> Self {
        let mut cpu = SM83 {
            pc: 0x0100, //program counter
            sp: 0xFFFE, //stack counter
            af: 0,      //accumalator and flags

            bc: 0,
            de: 0,

            hl: 0, //HL Register
            ir: 0, //instruction register
            ie: 0, //interrupt enable
        };
        cpu.set_register(RegisterName::A, 0x11);
        cpu.set_register(RegisterName::D, 0xFF);
        cpu.set_register(RegisterName::H, 0x56);
        cpu.set_register(RegisterName::L, 0x0D);
        cpu.set_flag(RegisterFlag::Zero);
        cpu
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
        }
    }

    fn set_flag(&mut self, flag: RegisterFlag) {
        self.af |= 1 << (flag as u8);
    }

    fn get_flag(&mut self, flag: RegisterFlag) -> bool {
        (self.af & 1 << (flag as u8)) != 0
    }

    fn clear_flag(&mut self, flag: RegisterFlag) {
        self.af &= !(1 << flag as u8);
    }

    fn dispatch_block_0(&mut self, bus: &mut Bus, opcode: u8) {
        assert!(opcode >> 6 == 0b00);
        if opcode == 0b00000000 {
            return;
        } else if opcode == 0b00010000 {
            todo!("stop");
        }
        let last_4 = opcode & 0b1111;
        match last_4 {
            0b0001 => todo!("ld r16 imm16"),
            0b0010 => todo!("ld [r16mem] a"),
            0b1010 => todo!("ld a [r16mem]"),
            0b1000 => todo!("ld [imm16] sp"),
            0b0011 => todo!("inc r16"),
            0b1011 => todo!("dec r16"),
            0b1001 => todo!("add hl, r16"),
            _ => {
                let last_3 = opcode & 0b111;
                match last_3 {
                    0b100 => todo!("inc r8"),
                    0b101 => todo!("dec r8"),
                    0b110 => todo!("ld r8 imm8"),
                    0b111 => {
                        let bit_543 = opcode >> 3 & 0b111;
                        match bit_543 {
                            0b000 => todo!("rlca"),
                            0b001 => todo!("rrca"),
                            0b010 => todo!("rla"),
                            0b011 => todo!("rra"),
                            0b100 => todo!("daa"),
                            0b101 => todo!("cpl"),
                            0b110 => todo!("scf"),
                            0b111 => todo!("ccf"),
                            _ => unreachable!(),
                        }
                    }
                    0b000 => {
                        let bit_43 = opcode >> 2 & 0b11;
                        match bit_43 {
                            0b11 => todo!("jr imm8"),
                            _ => todo!("jr cond imm8"),
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    //block 1
    fn load_register_register(&mut self, bus: &mut Bus, opcode: u8) {
        assert!(opcode >> 6 == 0b01);
        let source: RegisterR8 = (opcode & 0b111).try_into().unwrap();
        let dest: RegisterR8 = (opcode >> 3 & 0b111).try_into().unwrap();

        match (source, dest) {
            (RegisterR8::HL, RegisterR8::HL) => {
                todo!("implement halt instruction")
            }
            (src, dst) => {
                self.set_register(dst.into(), self.get_register(src.into()));
            }
        }
    }

    fn arithmetic_8bit(&mut self, bus: &mut Bus, opcode: u8) {
        assert!(opcode >> 6 == 0b10);
        let operation_type = opcode >> 3 & 0b111;
        match operation_type {
            0b000 => todo!("add r8"),
            0b001 => todo!("adc r8"),
            0b010 => todo!("sub r8"),
            0b011 => todo!("sbc r8"),
            0b100 => todo!("and r8"),
            0b101 => todo!("xor r8"),
            0b110 => todo!("or r8"),
            0b111 => todo!("cp r8"),
            _ => unreachable!("operation_type is three bit"),
        }
    }

    fn dispatch_block_3(&mut self, bus: &mut Bus, opcode: u8) {
        assert!(opcode >> 6 == 0b11);
        let last_3 = opcode & 0b111;
        if last_3 == 0b110 {
            let arithmetic_type = opcode >> 3 & 0b111;
            let _imm8 = self.fetch(bus);
            //arithetic imm8
            match arithmetic_type {
                0b000 => todo!("add"),
                0b001 => todo!("adc"),
                0b010 => todo!("sub"),
                0b011 => todo!("sbc"),
                0b100 => todo!("and"),
                0b101 => todo!("xor"),
                0b110 => todo!("or"),
                0b111 => todo!("cp"),
                _ => unreachable!("arithmetic_type is only 3bit"),
            }
        }
    }

    fn fetch(&mut self, bus: &mut Bus) -> u8 {
        let byte = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    pub fn step(&mut self, bus: &mut Bus) {
        let opcode = self.fetch(bus);
        let block = opcode >> 6;
        match block {
            0b00 => self.dispatch_block_0(bus, opcode),
            0b01 => self.load_register_register(bus, opcode),
            0b10 => self.arithmetic_8bit(bus, opcode),
            0b11 => self.dispatch_block_3(bus, opcode),
            _ => unreachable!("operation block type is 2bit"),
        }
    }
}
