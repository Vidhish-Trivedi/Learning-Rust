// Command-line arguments.
use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();          // First element is always the target path of the executable.

    let command = args[1].clone();

    println!("CL Args: {:?}", args);
    println!("Command: {:?}", command);

    let name = "Jack";
    let status = "100%";

    if command.to_lowercase() == "hello" {
        println!("Hi, {}", name);
    }
    else if command.to_lowercase() == "status"{
        println!("Status is: {}", status);
    }
    else{
        println!("Unrecognized command");
    }
}
