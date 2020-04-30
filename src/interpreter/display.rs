use crate::config::{SCREEN_X, SCREEN_Y};
use log::trace;

pub struct Display {
    pixel_states: [[bool; SCREEN_Y]; SCREEN_X],
}

impl Display {
    pub fn new() -> Self {
        Display {
            pixel_states: [[false; SCREEN_Y]; SCREEN_X],
        }
    }

    /// Draw the sprite starting on top left x and y coordinates. Return value is whether there was a collision.
    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        let mut collision = false;

        for i in 0..sprite.len() as u8 {
            trace!("\tSPRITE - {:8b}", sprite[i as usize]);
            for j in 0..8 as u8 {
                if sprite[i as usize] & (1 << (7 - j)) != 0 {
                    collision = collision | self.toggle_pixel(j.checked_add(x), i.checked_add(y));
                }
            }
        }

        collision
    }

    /// Toggle a pixel at x and y coordinates. Return value is whether there was a collision.
    fn toggle_pixel(&mut self, x_option: Option<u8>, y_option: Option<u8>) -> bool {
        if x_option.is_none() || y_option.is_none() {
            return false;
        }
        let x = x_option.expect("Error getting x value");
        let y = y_option.expect("Error getting y value");
        if x as usize >= self.pixel_states.len()
            || y as usize >= self.pixel_states[x as usize].len()
        {
            return false;
        }
        let previous_value = self.pixel_states[x as usize][y as usize];
        self.pixel_states[x as usize][y as usize] = !previous_value;
        return previous_value;
    }

    /// Get the current state of the display.
    pub fn state(&self) -> [[bool; SCREEN_Y]; SCREEN_X] {
        self.pixel_states
    }

    /// Reset the display to its initial state.
    pub fn reset(&mut self) {
        self.pixel_states = [[false; SCREEN_Y]; SCREEN_X];
    }
}
