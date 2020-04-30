use log::trace;

use core::fmt;
use std::fmt::{Display, Formatter};

pub trait Register<T>: fmt::Display {
    /// Current value of the register.
    fn value(&self) -> T;
    /// Write new value into the register.
    fn write_value(&mut self, val: T);
    /// Increment value and return the new value.
    fn increment_value_by(&mut self, increment_by: T) -> T;
    /// Decrement value and return the new value.
    fn decrement_value_by(&mut self, decrement_by: T) -> T;
    /// Get human-readable name of the register.
    fn name(&self) -> &str;
    /// Reset the register to its initial value.
    fn reset(&mut self);
}

pub struct Register8Bit {
    name: String,
    value: u8,
    initial_value: u8,
}

impl Register8Bit {
    pub fn new(name: &str) -> Self {
        Register8Bit {
            name: name.to_string(),
            value: 0,
            initial_value: 0,
        }
    }
}

impl Register<u8> for Register8Bit {
    fn value(&self) -> u8 {
        trace!("\tREGISTER - Read {} = {:X}", self.name, self.value);
        self.value
    }

    fn write_value(&mut self, val: u8) {
        trace!("\tREGISTER - Write {} = {:X}", self.name, val);
        self.value = val
    }

    fn increment_value_by(&mut self, increment_by: u8) -> u8 {
        self.value = self.value + increment_by;
        trace!("\tREGISTER - Increment {} by {:X} = {:X}", self.name, increment_by, self.value);
        self.value
    }

    fn decrement_value_by(&mut self, decrement_by: u8) -> u8 {
        self.value = self.value - decrement_by;
        trace!("\tREGISTER - Decrement {} by {:X} = {:X}", self.name, decrement_by, self.value);
        self.value
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn reset(&mut self) {
        trace!("\tREGISTER - Reset {} to {}", self.name, self.initial_value);
        self.value = self.initial_value;
    }
}

impl Display for Register8Bit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Register (8-bit) {}: {:X}", self.name, self.value)
    }
}

pub struct Register16Bit {
    name: String,
    value: u16,
    initial_value: u16,
}

impl Register16Bit {
    pub fn new(name: &str) -> Self {
        Register16Bit {
            name: name.to_string(),
            value: 0,
            initial_value: 0,
        }
    }
}

impl Register<u16> for Register16Bit {
    fn value(&self) -> u16 {
        trace!("\tREGISTER - Read {} = {:X}", self.name, self.value);
        self.value
    }

    fn write_value(&mut self, val: u16) {
        trace!("\tREGISTER - Write {} = {:X}", self.name, val);
        self.value = val
    }

    fn increment_value_by(&mut self, increment_by: u16) -> u16 {
        self.value = self.value + increment_by;
        trace!("\tREGISTER - Increment {} by {:X} = {:X}", self.name, increment_by, self.value);
        self.value
    }

    fn decrement_value_by(&mut self, decrement_by: u16) -> u16 {
        self.value = self.value - decrement_by;
        trace!("\tREGISTER - Decrement {} by {:X} = {:X}", self.name, decrement_by, self.value);
        self.value
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn reset(&mut self) {
        self.value = self.initial_value;
        trace!("\tREGISTER - Reset {} to {:X}", self.name, self.initial_value);
    }
}

impl Display for Register16Bit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Register (16-bit) {}: {:X}", self.name, self.value)
    }
}
