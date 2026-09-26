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

    fn load_register_register(&mut self, r1: RegisterName, r2: RegisterName) {
        let x = self.get_register(r1);
        &mut self.set_register(r2, x);
    }

    fn load_register_immediate(&mut self, r1: RegisterName, value: u16) {
        self.set_register(r1, value);
    }
}
