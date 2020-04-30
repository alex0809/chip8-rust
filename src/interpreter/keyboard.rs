use log::trace;

pub const NUMBER_OF_KEYS: u8 = 16;

pub struct Keyboard {
    key_states: [bool; NUMBER_OF_KEYS as usize],
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard { key_states: [false; NUMBER_OF_KEYS as usize] }
    }

    pub fn key_state(&mut self, key: u8) -> bool {
        trace!("\tKEYBOARD - Read key {} = {}", key, self.key_states[key as usize]);
        let state = self.key_states[key as usize];
        if state {
            self.key_states[key as usize] = false;
        }
        state
    }

    /// Set key as pressed.
    pub fn key_pressed(&mut self, key: u8) {
        trace!("\tKEYBOARD - Setting key {} = pressed", key);
        self.key_states[key as usize] = true;
    }

    /// Set key as released.
    pub fn key_released(&mut self, key: u8) {
        trace!("\tKEYBOARD - Read key {} = released", key);
        self.key_states[key as usize] = false;
    }

    /// Reset the keyboard state.
    pub fn reset(&mut self) {
        self.key_states = [false; NUMBER_OF_KEYS as usize];
    }
}