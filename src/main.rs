use std::env;

mod terminal_io;

// cargo run -- arg1 arg2 arg3
fn main() {
    println!("HelloRust!");

    let args: Vec<String> = env::args().collect();
    for arg in &args {
        let index = args.iter().position(|x| x == arg).unwrap();
        println!("Argument {}: {}", index, arg);
    }

    terminal_io::print_to_terminal("Hello from main.rs");
    let str = terminal_io::read_string_from_terminal();
    terminal_io::print_to_terminal(&str);
    let int = terminal_io::read_int_from_terminal();
    match int {
        Ok(value) => terminal_io::print_to_terminal(&value.to_string()),
        Err(e) => terminal_io::print_to_terminal(&format!("Error: {:?}", e)),
    }
    let double = terminal_io::read_double_from_terminal();
    match double {
        Ok(value) => terminal_io::print_to_terminal(&value.to_string()),
        Err(e) => terminal_io::print_to_terminal(&format!("Error: {:?}", e)),
    }
}

