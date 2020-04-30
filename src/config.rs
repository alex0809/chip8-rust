use sdl2::keyboard::Keycode;

pub const DEFAULT_REFRESH_RATE: u32 = 60;

pub const SCREEN_X: usize = 64;
pub const SCREEN_Y: usize = 32;

pub const KEYPAD_1: Keycode = Keycode::Num2;
pub const KEYPAD_2: Keycode = Keycode::Num3;
pub const KEYPAD_3: Keycode = Keycode::Num4;
pub const KEYPAD_C: Keycode = Keycode::Num5;
pub const KEYPAD_4: Keycode = Keycode::W;
pub const KEYPAD_5: Keycode = Keycode::E;
pub const KEYPAD_6: Keycode = Keycode::R;
pub const KEYPAD_D: Keycode = Keycode::T;
pub const KEYPAD_7: Keycode = Keycode::S;
pub const KEYPAD_8: Keycode = Keycode::D;
pub const KEYPAD_9: Keycode = Keycode::F;
pub const KEYPAD_E: Keycode = Keycode::G;
pub const KEYPAD_A: Keycode = Keycode::X;
pub const KEYPAD_0: Keycode = Keycode::C;
pub const KEYPAD_B: Keycode = Keycode::V;
pub const KEYPAD_F: Keycode = Keycode::B;

pub const RESET_KEY: Keycode = Keycode::Backspace;
pub const INSTRUCTION_STEP_KEY: Keycode = Keycode::Space;
pub const EXIT_KEY: Keycode = Keycode::Escape;