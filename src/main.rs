use std::fs::File;

use crate::interpreter::Interpreter;
use clap::{Arg, App};
use log::LevelFilter;

mod instruction;
mod interpreter;
mod sdl2;
mod logger;
mod config;


fn main() {
    let matches = App::new("Chip-8 Interpreter")
        .version("0.1")
        .author("Alex Friesenhahn")
        .arg(Arg::with_name("filename")
            .help("the file to load")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("verbosity")
            .help("the verbosity of the system out; debug logs out all executed instructions, trace adds logs of all register and memory accesses")
            .short("v")
            .required(false)
            .possible_values(&["info", "debug", "trace"])
            .default_value("info"))
        .arg(Arg::with_name("step_mode")
            .help("If step mode is enabled, the interpreter processes one instruction each time the SPACE key is pressed")
            .short("s")
            .required(false)
            .takes_value(false)
        )
    .get_matches();

    let file_name = matches.value_of("filename").unwrap();
    let log_level = match matches.value_of("verbosity").unwrap() {
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => unreachable!()
    };
    let step_mode = matches.is_present("step_mode");

    logger::init(log_level);

    let mut interpreter = Interpreter::new();
    interpreter.load_program_file(&mut File::open(file_name).expect("Could not open file"));
    sdl2::for_interpreter(&mut interpreter, step_mode).expect("Error!");
}
