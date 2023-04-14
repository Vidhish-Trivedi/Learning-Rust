// Variables hold primitive data or references to data.
// Variables are immutable by default.
// Rust is a block-scoped language.

pub fn run() {
    // Define constants.
    // Need to explicitly define type.
    const ID: i32 = 001;
    println!("ID: {}", ID);        // Will print: 1

    let name = "Jack";      // Similar to const.        IMPORTANT.
    let mut age = 25;       // This is how we can make a variable mutable.

    // name = "John";           // Variables are immutable by default.

    println!("My name is {}, I am {} years old.", name, age);

    age = 38;

    println!("My name is {}, I am {} years old.", name, age);

    // Assign multiple variables at once.
    let (my_name, my_age) = ("James", 24);
    println!("{} is {}", my_name, my_age);
}