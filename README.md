# CHIP-8 Interpreter

This is a CHIP-8 interpreter written in Rust.

## Requirements

Requires sdl2 and sdl2-gfx libraries installed.

## Usage

This repository does not contain any games to run on the interpreter.
I tested this interpreter on the ROMs found on [this site](https://www.zophar.net/pdroms/chip8.html).

To start the interpreter, run it with a CHIP-8 ROM file:
`chip8 ROM_FILE`

Additional parameters are available, including verbose debug output and 
a step-mode for executing instructions only on press of a key. List them with
`chip8 --help`

## Controls
The 12-key keypad that the CHIP-8 requires is mapped to:

| | | | |
| --- | --- | --- | --- |
| 2 | 3 | 4 | 5 |
| W | E | R | T |
| S | D | F | G |
| X | C | V | B |

Other key mappings:
* `Backspace`: Reset interpreter
* `Space`: Execute next instruction in step mode
* `Escape`: Quit
