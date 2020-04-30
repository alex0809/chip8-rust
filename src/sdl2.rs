extern crate sdl2;

use std::thread::sleep;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::audio::{AudioCallback, AudioSpecDesired};

use crate::interpreter::Interpreter;
use crate::config::*;
use self::sdl2::audio::AudioDevice;

const WHITE: Color = Color::RGB(255, 255, 255);
const BLACK: Color = Color::RGB(0, 0, 0);

pub fn for_interpreter(interpreter: &mut Interpreter, step_mode: bool) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let audio_subsystem = sdl_context.audio()?;
    let audio_device = prepare_sound(&audio_subsystem);

    let window = video_subsystem
        .window("Chip-8", 640, 320)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_logical_size(SCREEN_X as u32, SCREEN_Y as u32).map_err(|e| e.to_string())?;
    canvas.set_draw_color(BLACK);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    let mut pixel_states;
    let mut interpreter_time = 0;
    let start_time = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }

                Event::KeyDown {
                    keycode: Some(key),
                    ..
                } => {
                    match key {
                        EXIT_KEY => {
                            break 'running;
                        }
                        RESET_KEY => {
                            interpreter.reset();
                        }
                        INSTRUCTION_STEP_KEY => {
                            if step_mode {
                                interpreter.instruction_step();
                            }
                        }
                        KEYPAD_1 => { interpreter.key_pressed(1) }
                        KEYPAD_2 => { interpreter.key_pressed(2) }
                        KEYPAD_3 => { interpreter.key_pressed(3) }
                        KEYPAD_C => { interpreter.key_pressed(0xC) }
                        KEYPAD_4 => { interpreter.key_pressed(4) }
                        KEYPAD_5 => { interpreter.key_pressed(5) }
                        KEYPAD_6 => { interpreter.key_pressed(6) }
                        KEYPAD_D => { interpreter.key_pressed(0xD) }
                        KEYPAD_7 => { interpreter.key_pressed(7) }
                        KEYPAD_8 => { interpreter.key_pressed(8) }
                        KEYPAD_9 => { interpreter.key_pressed(9) }
                        KEYPAD_E => { interpreter.key_pressed(0xE) }
                        KEYPAD_A => { interpreter.key_pressed(0xA) }
                        KEYPAD_0 => { interpreter.key_pressed(0) }
                        KEYPAD_B => { interpreter.key_pressed(0xB) }
                        KEYPAD_F => { interpreter.key_pressed(0xF) }
                        _ => {}
                    }
                }

                Event::KeyUp {
                    keycode: Some(key),
                    ..
                } => {
                    match key {
                        KEYPAD_1 => { interpreter.key_released(1) }
                        KEYPAD_2 => { interpreter.key_released(2) }
                        KEYPAD_3 => { interpreter.key_released(3) }
                        KEYPAD_C => { interpreter.key_released(0xC) }
                        KEYPAD_4 => { interpreter.key_released(4) }
                        KEYPAD_5 => { interpreter.key_released(5) }
                        KEYPAD_6 => { interpreter.key_released(6) }
                        KEYPAD_D => { interpreter.key_released(0xD) }
                        KEYPAD_7 => { interpreter.key_released(7) }
                        KEYPAD_8 => { interpreter.key_released(8) }
                        KEYPAD_9 => { interpreter.key_released(9) }
                        KEYPAD_E => { interpreter.key_released(0xE) }
                        KEYPAD_A => { interpreter.key_released(0xA) }
                        KEYPAD_0 => { interpreter.key_released(0) }
                        KEYPAD_B => { interpreter.key_released(0xB) }
                        KEYPAD_F => { interpreter.key_released(0xF) }
                        _ => {}
                    }
                }

                _ => {}
            }
        }
        if !step_mode {
            let now = start_time.elapsed().as_micros();

            while now > interpreter_time {
                let step_result = interpreter.instruction_step();
                interpreter_time = interpreter_time + step_result.time_passed() as u128;
            }
        }

        interpreter.frequency_step();
        pixel_states = interpreter.pixel_states();

        if interpreter.sound_on() {
            audio_device.resume();
        } else {
            audio_device.pause();
        }

        redraw_screen(&mut canvas, pixel_states).expect("Failed to redraw screen");
        sleep(Duration::new(0, 1_000_000_000u32 / DEFAULT_REFRESH_RATE));
    }

    Ok(())
}

fn redraw_screen(
    canvas: &mut Canvas<Window>,
    pixel_states: [[bool; SCREEN_Y]; SCREEN_X],
) -> Result<(), String> {
    for i in 0..pixel_states.len() {
        for j in 0..pixel_states[i].len() {
            if pixel_states[i][j] {
                canvas.set_draw_color(WHITE)
            } else {
                canvas.set_draw_color(BLACK)
            }
            canvas.draw_point(Point::new(i as i32, j as i32))?;
        }
    }
    canvas.present();
    Ok(())
}

fn prepare_sound(audio_subsystem: &sdl2::AudioSubsystem) -> AudioDevice<SquareWave> {
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None,
    };

    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        SquareWave {
            phase_inc: 440.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.25,
        }
    }).unwrap();

    device.pause();
    device
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

