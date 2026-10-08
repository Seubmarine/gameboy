// documentation
// https://gbdev.io/pandocs/CPU_Instruction_Set.html
// https://gbdev.io/gb-opcodes/optables/
// https://gbdev.io/gb-opcodes/optables/octal

pub struct SM83 {
    pc: u16, //program counter
    sp: u16, //stack counter
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
    pub fn new() -> Self {
        SM83 {
            pc: 0, //program counter
            sp: 0, //stack counter
            af: 0, //accumalator and flags

            bc: 0,
            de: 0,

            hl: 0, //HL Register
            ir: 0, //instruction register
            ie: 0, //interrupt enable
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

    //block 1
    fn load_register_register(&mut self, opcode: u8) {
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
}
