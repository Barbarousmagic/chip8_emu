const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const START_ADDR: u16 = 0x200;
const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, //0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Cpu {
    ram: [u8; RAM_SIZE],
    v_reg: [u8; NUM_REGS],
    pc: u16,
    i_reg: u16,
    stack: [u16; STACK_SIZE],
    sp: u16,
    delay_timer: u8,
    sound_timer: u8,
    keys: [bool; NUM_KEYS],
    screen: [bool; SCREEN_HEIGHT * SCREEN_WIDTH],
}

impl Cpu {
    pub fn new() -> Self {
        let mut new_cpu = Self {
            ram: [0; RAM_SIZE],
            v_reg: [0; NUM_REGS],
            pc: START_ADDR,
            i_reg: 0,
            stack: [0; STACK_SIZE],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keys: [false; NUM_KEYS],
            screen: [false; SCREEN_HEIGHT * SCREEN_WIDTH],
        };
        new_cpu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        new_cpu
    }

    pub fn reset(&mut self) {
        self.ram = [0; RAM_SIZE];
        self.v_reg = [0; NUM_REGS];
        self.pc = START_ADDR;
        self.i_reg = 0;
        self.stack = [0; STACK_SIZE];
        self.sp = 0;
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.keys = [false; NUM_KEYS];
        self.screen = [false; SCREEN_HEIGHT * SCREEN_WIDTH];
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;

    }
    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
    fn fetch(&mut self) -> u16 {
        let higher_byte: u8 = self.ram[self.pc as usize];
        let lower_byte: u8 = self.ram[self.pc as usize +1];
        ((higher_byte as u16) << 8) | (lower_byte as u16)
    }
}