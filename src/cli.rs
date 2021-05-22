use std::env;

pub fn run() {
    let name = "Sacha";

    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    if command == "hello" {
        println!("Hello {} !", name);
    } else if command == "hello-world" {
        println!("Hello World \\o/");
    } else {
        println!("That is not a valid command.")
    }
}
