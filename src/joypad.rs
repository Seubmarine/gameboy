#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum Button {
    A = 1 << 0,
    B = 1 << 1,
    Select = 1 << 2,
    Start = 1 << 3,
    Right = 1 << 4,
    Left = 1 << 5,
    Up = 1 << 6,
    Down = 1 << 7,
}

impl Button {
    fn button_pressed_flag(self) -> u8 {
        match self {
            Button::A => 1 << 0,
            Button::B => 1 << 1,
            Button::Select => 1 << 2,
            Button::Start => 1 << 3,
            Button::Right => 1 << 4,
            Button::Left => 1 << 5,
            Button::Up => 1 << 6,
            Button::Down => 1 << 7,
        }
    }
}

pub struct Joypad {
    button_pressed: u8,
    select_buttons: bool,
    select_directional: bool,
    previous_state: u8, //interrupt handling
}

impl Joypad {
    pub fn new() -> Self {
        Joypad {
            button_pressed: 0,
            select_buttons: false,
            select_directional: false,
            previous_state: 0b1111,
        }
    }

    pub fn press(&mut self, button: Button) {
        self.button_pressed |= button.button_pressed_flag();
    }

    pub fn unpress(&mut self, button: Button) {
        self.button_pressed &= !button.button_pressed_flag();
    }

    pub fn unpress_all(&mut self) {
        self.button_pressed = 0;
    }

    const SELECT_BUTTON_FLAG: u8 = 1 << 5;
    const SELECT_DIRECTION_FLAG: u8 = 1 << 4;
    const P1_BUTTON_MASK: u8 = 0b1111;

    pub fn write_p1(&mut self, value: u8) {
        self.select_buttons = value & Joypad::SELECT_BUTTON_FLAG == 0;
        self.select_directional = value & Joypad::SELECT_DIRECTION_FLAG == 0;
    }

    pub fn read_p1(&mut self) -> u8 {
        let buttons: u8 = if self.select_buttons {
            Joypad::SELECT_BUTTON_FLAG | self.button_pressed & Joypad::P1_BUTTON_MASK
        } else {
            0
        };

        let direction: u8 = if self.select_directional {
            Joypad::SELECT_DIRECTION_FLAG | self.button_pressed >> 4 & Joypad::P1_BUTTON_MASK
        } else {
            0
        };

        !(buttons | direction)
    }

    pub fn take_interrupt(&mut self) -> bool {
        let current_state = self.read_p1() & Joypad::P1_BUTTON_MASK;
        for i in 0..4 {
            let prev_button = self.previous_state >> i & 0b1;
            let curr_button = current_state >> i & 0b1;
            if prev_button == 1 && curr_button == 0 {
                self.previous_state = current_state;
                return true;
            }
        }
        self.previous_state = current_state;
        false
    }
}

#[test]
fn p1() {
    //all the bits in p1 are inversed, an activated mode is 0
    //a pressed button is 0
    let mut j = Joypad::new();
    assert_eq!(j.read_p1(), 0b11111111); //default state (everything disabled) is everything at 1
    j.press(Button::A);
    assert_eq!(j.read_p1(), 0b11111111); //still set to 0xFF since button mode is not activated
    j.write_p1(!Joypad::SELECT_BUTTON_FLAG);
    assert_eq!(j.read_p1(), 0b11011110); //bit 5 is select button mode, bit 0 is button A
    j.press(Button::B);
    assert_eq!(j.read_p1(), 0b11011100); //bit 1 button B
    j.write_p1(0xFF);
    assert_eq!(j.read_p1(), 0b11111111); //disable select button mode, everything is disabled
    j.write_p1(!Joypad::SELECT_DIRECTION_FLAG);
    assert_eq!(j.read_p1(), 0b11101111); //bit 4 is select dir mode
    j.press(Button::Up);
    assert_eq!(j.read_p1(), 0b11101011); //bit 2 is Dir Up
    j.press(Button::Down);
    assert_eq!(j.read_p1(), 0b11100011); //bit 3 is Dir Down
    j.write_p1(!(Joypad::SELECT_DIRECTION_FLAG | Joypad::SELECT_BUTTON_FLAG));
    assert_eq!(j.read_p1(), 0b11000000); //two mode selected, and multiple button selected
}

#[test]
fn p1_different_button_same_bit() {
    //all the bits in p1 are inversed, an activated mode is 0
    //a pressed button is 0
    let mut j = Joypad::new();
    j.write_p1(!(Joypad::SELECT_DIRECTION_FLAG | Joypad::SELECT_BUTTON_FLAG));
    j.press(Button::A);
    j.press(Button::Right);
    assert_eq!(j.read_p1(), 0b11001110); //two mode selected, and multiple button selected
}

#[test]
fn interrupt() {
    let mut j = Joypad::new();
    j.write_p1(!Joypad::SELECT_BUTTON_FLAG);
    j.press(Button::A);
    assert_eq!(j.take_interrupt(), true);
    assert_eq!(j.take_interrupt(), false); //only trigger once after press
    j.unpress(Button::A);
    assert_eq!(j.take_interrupt(), false);
    j.unpress(Button::A);
    assert_eq!(j.take_interrupt(), false);

    j.press(Button::A);
    assert_eq!(j.take_interrupt(), true);
    j.press(Button::B);
    assert_eq!(j.take_interrupt(), true);
    j.unpress(Button::A);
    assert_eq!(j.take_interrupt(), false);
    j.unpress(Button::A);
    assert_eq!(j.take_interrupt(), false);
}

#[test]
fn interrupt_different_button_same_bit() {
    let mut j = Joypad::new();
    j.write_p1(!(Joypad::SELECT_BUTTON_FLAG | Joypad::SELECT_DIRECTION_FLAG));
    j.press(Button::A);
    assert_eq!(j.take_interrupt(), true);
    j.press(Button::Right);
    // did not change value from high to low, because button A was already pressed,
    // and the two mode are activated
    assert_eq!(j.take_interrupt(), false);
}
