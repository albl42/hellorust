use std::env;

// cargo run -- arg1 arg2 arg3
fn main() {
    println!("HelloRust!");

    let args: Vec<String> = env::args().collect();
    for arg in &args {
        let index = args.iter().position(|x| x == arg).unwrap();
        println!("Argument {}: {}", index, arg);
    }
}
