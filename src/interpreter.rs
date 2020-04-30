extern crate rand;

use log::{debug, trace};

use rand::Rng;

use std::fs::File;
use std::io::Read;

use display::Display;
use keyboard::Keyboard;
use memory::Memory;
use stack::Stack;

use crate::config::{SCREEN_X, SCREEN_Y};
use crate::instruction::Instruction;
use crate::interpreter::keyboard::NUMBER_OF_KEYS;
use crate::interpreter::register::{Register, Register16Bit, Register8Bit};

mod display;
mod keyboard;
mod memory;
mod register;
mod stack;

/// Holds the full state of an interpreter.
pub struct Interpreter {
    memory: Memory,
    stack: Stack,
    display: Display,
    keyboard: Keyboard,
    v_registers: [Register8Bit; 16],
    i_register: Register16Bit,
    delay_register: Register8Bit,
    sound_register: Register8Bit,
    program_counter: Register16Bit,
    waiting_for_key_press: bool,
    key_press_result_register: u8,
    loaded_program: Vec<u8>,
}

/// The result of a single command execution.
pub struct StepResult {
    time_passed: u128,
}

impl StepResult {
    /// The time (in microseconds) that the last step took
    pub fn time_passed(&self) -> u128 {
        self.time_passed
    }
}

impl Interpreter {
    /// Create a new interpreter with all memory locations and registers in
    /// their default state.
    pub fn new() -> Self {
        Interpreter {
            memory: Memory::new(),
            stack: Stack::new(),
            display: Display::new(),
            keyboard: Keyboard::new(),
            v_registers: [
                Register8Bit::new("V0"),
                Register8Bit::new("V1"),
                Register8Bit::new("V2"),
                Register8Bit::new("V3"),
                Register8Bit::new("V4"),
                Register8Bit::new("V5"),
                Register8Bit::new("V6"),
                Register8Bit::new("V7"),
                Register8Bit::new("V8"),
                Register8Bit::new("V9"),
                Register8Bit::new("VA"),
                Register8Bit::new("VB"),
                Register8Bit::new("VC"),
                Register8Bit::new("VD"),
                Register8Bit::new("VE"),
                Register8Bit::new("VF"),
            ],
            i_register: Register16Bit::new("I"),
            delay_register: Register8Bit::new("Delay"),
            sound_register: Register8Bit::new("Sound"),
            program_counter: Register16Bit::new("PC"),
            waiting_for_key_press: false,
            key_press_result_register: 0,
            loaded_program: Vec::new(),
        }
    }

    /// Indicate to the interpreter that a key has been pressed.
    pub fn key_pressed(&mut self, key: u8) {
        self.keyboard.key_pressed(key);
    }

    /// Indicate to the interpreter that a key has been released.
    pub fn key_released(&mut self, key: u8) {
        self.keyboard.key_released(key);
    }

    /// Get the current state of all pixels of the screen.
    pub fn pixel_states(&self) -> [[bool; SCREEN_Y]; SCREEN_X] {
        self.display.state()
    }

    /// Whether sound should  currently be on.
    pub fn sound_on(&self) -> bool {
        self.sound_register.value() > 0
    }

    /// Load a program via given file into memory.
    pub fn load_program_file(&mut self, file: &mut File) {
        let mut data = Vec::new();
        file.read_to_end(&mut data).expect("Could not read file");
        self.loaded_program = data;
        self.load_program(&self.loaded_program.clone());
    }

    /// Reset the interpreter to its original state, then load the program that was
    /// initially loaded if any.
    pub fn reset(&mut self) {
        self.memory.reset();
        self.stack.reset();
        self.v_registers.iter_mut().for_each(|r| r.reset());
        self.i_register.reset();
        self.delay_register.reset();
        self.sound_register.reset();
        self.program_counter.reset();
        self.display.reset();
        self.keyboard.reset();
        self.waiting_for_key_press = false;
        self.key_press_result_register = 0;

        self.load_program(&self.loaded_program.clone());
    }

    /// This should be called with 60HZ frequency to set the sound and delay registers
    /// to their correct states.
    pub fn frequency_step(&mut self) {
        if self.sound_register.value() > 0 {
            self.sound_register.decrement_value_by(1);
        }

        if self.delay_register.value() > 0 {
            self.delay_register.decrement_value_by(1);
        }
    }

    /// Process the next instruction. The result details how long this clock cycle should take.
    pub fn instruction_step(&mut self) -> StepResult {
        if self.waiting_for_key_press {
            debug!("Executing: Wait for key press");
            for i in 0..NUMBER_OF_KEYS {
                if self.keyboard.key_state(i) {
                    debug!(
                        "Key {:X} is pressed! Writing value to V_{:X}",
                        i, self.key_press_result_register
                    );
                    self.v_registers[self.key_press_result_register as usize].write_value(i as u8);
                    self.waiting_for_key_press = false;
                }
            }
            return StepResult { time_passed: 100 };
        }

        let instruction =
            Instruction::parse(self.memory.two_byte_read(self.program_counter.value()));
        debug!(
            "{:4X} - Executing: {}",
            self.program_counter.value(),
            instruction
        );
        self.program_counter.increment_value_by(2);
        trace!("\t---");

        let time_passed = match instruction {
            Instruction::ClearDisplay => {
                self.display.reset();
                109
            }
            Instruction::Return => {
                self.program_counter.write_value(self.stack.pop());
                105
            }
            Instruction::JumpToAddress(nnn) => {
                self.program_counter.write_value(nnn);
                105
            }
            Instruction::CallAddress(nnn) => {
                self.stack.push(self.program_counter.value());
                self.program_counter.write_value(nnn);
                105
            }
            Instruction::SkipIfVxEqualKk(x, kk) => {
                if self.v_registers[x as usize].value() == kk {
                    self.program_counter.increment_value_by(2);
                }
                55
            }
            Instruction::SkipIfVxNotEqualKk(x, kk) => {
                if self.v_registers[x as usize].value() != kk {
                    self.program_counter.increment_value_by(2);
                }
                55
            }
            Instruction::SkipIfVxEqualVy(x, y) => {
                if self.v_registers[x as usize].value() == self.v_registers[y as usize].value() {
                    self.program_counter.increment_value_by(2);
                }
                73
            }
            Instruction::LoadVxKk(x, kk) => {
                self.v_registers[x as usize].write_value(kk);
                27
            }
            Instruction::AddVxKk(x, kk) => {
                let x_value = self.v_registers[x as usize].value();
                let (new_value, _) = x_value.overflowing_add(kk);
                self.v_registers[x as usize].write_value(new_value);
                45
            }
            Instruction::LoadVxVy(x, y) => {
                self.v_registers[x as usize].write_value(self.v_registers[y as usize].value());
                200
            }
            Instruction::OrVxVy(x, y) => {
                let result =
                    self.v_registers[x as usize].value() | self.v_registers[y as usize].value();
                self.v_registers[x as usize].write_value(result);
                200
            }
            Instruction::AndVxVy(x, y) => {
                let result =
                    self.v_registers[x as usize].value() & self.v_registers[y as usize].value();
                self.v_registers[x as usize].write_value(result);
                200
            }
            Instruction::XorVxVy(x, y) => {
                let result =
                    self.v_registers[x as usize].value() ^ self.v_registers[y as usize].value();
                self.v_registers[x as usize].write_value(result);
                200
            }
            Instruction::AddVxVy(x, y) => {
                let (result, carry_bit) = self.v_registers[x as usize]
                    .value()
                    .overflowing_add(self.v_registers[y as usize].value());
                self.v_registers[0xF].write_value(carry_bit as u8);
                self.v_registers[x as usize].write_value(result);
                200
            }
            Instruction::SubVxVy(x, y) => {
                let x_value = self.v_registers[x as usize].value();
                let y_value = self.v_registers[y as usize].value();
                let (result, _) = x_value.overflowing_sub(y_value);
                if x_value > y_value {
                    self.v_registers[0xF].write_value(1);
                } else {
                    self.v_registers[0xF].write_value(0);
                }
                self.v_registers[x as usize].write_value(result);
                200
            }
            Instruction::ShiftRight(x, _) => {
                let value = self.v_registers[x as usize].value();
                self.v_registers[0xF].write_value(value & 1);
                self.v_registers[x as usize].write_value(value >> 1);
                200
            }
            Instruction::SubNVxVy(x, y) => {
                let value =
                    self.v_registers[x as usize].value() - self.v_registers[y as usize].value();
                if value > 0 {
                    self.v_registers[0xF].write_value(1);
                } else {
                    self.v_registers[0xF].write_value(0);
                }
                self.v_registers[x as usize].write_value(value);
                200
            }
            Instruction::ShiftLeft(x, _) => {
                let value = self.v_registers[x as usize].value();
                self.v_registers[0xF].write_value(value >> 7);
                self.v_registers[x as usize].write_value(value << 1);
                200
            }
            Instruction::SkipIfVxNotEqualVy(x, y) => {
                if self.v_registers[x as usize].value() != self.v_registers[y as usize].value() {
                    self.program_counter.increment_value_by(2);
                }
                73
            }
            Instruction::LoadAddr(nnn) => {
                self.i_register.write_value(nnn);
                55
            }
            Instruction::JumpToAddressPlusV0(nnn) => {
                let value = nnn + self.v_registers[0].value() as u16;
                self.program_counter.write_value(value);
                105
            }
            Instruction::RandomAnd(x, kk) => {
                let value: u8 = kk & rand::thread_rng().gen::<u8>();
                self.v_registers[x as usize].write_value(value);
                164
            }
            Instruction::DrawVxVyN(x, y, n) => {
                let mut sprite = Vec::with_capacity(n as usize);
                let memory_start = self.i_register.value();
                for i in 0..n {
                    sprite.push(self.memory.byte_read(memory_start + i as u16));
                }

                let x_coord = self.v_registers[x as usize].value();
                let y_coord = self.v_registers[y as usize].value();
                let collision = self.display.draw_sprite(x_coord, y_coord, &sprite);
                self.v_registers[0xF].write_value(collision as u8);
                10000 + 1000 * n as u128
            }
            Instruction::SkipIfKeyPressed(x) => {
                let key = self.v_registers[x as usize].value();
                if self.keyboard.key_state(key) {
                    self.program_counter.increment_value_by(2);
                }
                73
            }
            Instruction::SkipIfKeyNotPressed(x) => {
                let key = self.v_registers[x as usize].value();
                if !self.keyboard.key_state(key) {
                    self.program_counter.increment_value_by(2);
                }
                73
            }
            Instruction::LoadDelayTimer(x) => {
                let value = self.delay_register.value();
                self.v_registers[x as usize].write_value(value);
                45
            }
            Instruction::WaitForKey(x) => {
                self.waiting_for_key_press = true;
                self.key_press_result_register = x;
                0
            }
            Instruction::SetDelayTimer(x) => {
                let value = self.v_registers[x as usize].value();
                self.delay_register.write_value(value);
                45
            }
            Instruction::SetSoundTimer(x) => {
                let value = self.v_registers[x as usize].value();
                self.sound_register.write_value(value);
                45
            }
            Instruction::AddVxToI(x) => {
                let (value, _) = (self.v_registers[x as usize].value() as u16)
                    .overflowing_add(self.i_register.value());
                self.i_register.write_value(value);
                86
            }
            Instruction::LoadSpriteLocationToI(x) => {
                let digit = self.v_registers[x as usize].value();
                self.i_register.write_value((5 * digit) as u16);
                91
            }
            Instruction::LoadBcdToI(x) => {
                let location = self.i_register.value();
                let value = self.v_registers[x as usize].value();
                let hundreds = (value / 100) % 10;
                let tens = (value / 10) % 10;
                let ones = value % 10;
                self.memory.byte_write(location, hundreds);
                self.memory.byte_write(location + 1, tens);
                self.memory.byte_write(location + 2, ones);
                927
            }
            Instruction::LoadV0ThroughVxToI(x) => {
                let mut location = self.i_register.value();
                for i in 0..x + 1 {
                    let value = self.v_registers[i as usize].value();
                    self.memory.byte_write(location, value);
                    location = location + 1;
                }
                605
            }
            Instruction::LoadIToV0ThroughVx(x) => {
                let mut location = self.i_register.value();
                for i in 0..x + 1 {
                    let value = self.memory.byte_read(location);
                    self.v_registers[i as usize].write_value(value);
                    location = location + 1;
                }
                605
            }
            Instruction::Invalid(a, b, c, d) => panic!(format!(
                "Invalid instruction! Tried to execute: {:X}-{:X}-{:X}-{:X}",
                a, b, c, d
            )),
        };
        trace!("\t---");
        trace!("---");
        StepResult { time_passed }
    }

    /// Load a program given as byte-vector into memory.
    fn load_program(&mut self, program: &Vec<u8>) {
        self.memory.bytes_write(0x200, &program);
        self.program_counter.write_value(0x200);
    }
}
