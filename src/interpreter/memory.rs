use log::trace;

pub struct Memory {
    ram: [u8; 4096],
}

impl Memory {
    /// Create new memory with its initial state, including
    /// initializing the interpreter-exclusive memory.
    pub fn new() -> Self {
        let mut memory = Memory { ram: [0; 4096] };
        memory.reset();
        memory
    }

    /// Read the next two bytes from memory
    pub fn two_byte_read(&self, index: u16) -> (u8, u8) {
        trace!("\tMEMORY - Two byte read from {:X} = {:X}-{:X}", index, self.ram[index as usize], self.ram[(index+1) as usize]);
        (self.ram[index as usize], self.ram[(index + 1) as usize])
    }

    /// Read a byte from memory.
    pub fn byte_read(&self, index: u16) -> u8 {
        trace!("\tMEMORY - Byte read from {:X} = {:X}", index, self.ram[index as usize]);
        self.ram[index as usize]
    }

    /// Write to a byte in memory.
    pub fn byte_write(&mut self, index: u16, value: u8) {
        trace!("\tMEMORY - Byte write to {:X} = {:X}", index, value);
        self.ram[index as usize] = value
    }

    /// Write an array of bytes in memory, starting from start_index.
    pub fn bytes_write(&mut self, start_index: usize, bytes: &[u8]) {
        trace!("\tMEMORY - Bytes write to {:X} with length {:X}", start_index, bytes.len());
        for i in 0..bytes.len() {
            self.ram[i + start_index] = bytes[i];
        }
    }

    /// Reset the memory to its initial state
    pub fn reset(&mut self) {
        self.ram.iter_mut().for_each(|m| *m = 0);

        let sprites = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80,
            0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0,
            0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90,
            0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0,
            0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
            0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
        ];

        self.bytes_write(0, &sprites);
    }
}
