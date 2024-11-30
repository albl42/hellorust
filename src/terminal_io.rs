// Module: terminal_io
use std::io::{self, Write};

fn read_from_terminal(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

pub fn print_to_terminal(message: &str) {
    println!("{}", message);
}

pub fn read_int_from_terminal() -> Result<i32, std::num::ParseIntError> {
    let input = read_from_terminal("Please enter an int: ");
    input.parse::<i32>()
}

pub fn read_string_from_terminal() -> String {
    read_from_terminal("Please enter a string: ")
}

pub fn read_double_from_terminal() -> Result<f64, std::num::ParseFloatError> {
    let input = read_from_terminal("Please enter a double: ");
    input.parse::<f64>()
}
