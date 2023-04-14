pub fn run() {
    // Print with newline
    println!("Printing from print.rs");

    // Print without newline.
    // print!("This does not leave a line");

    // Basic formatting.
    println!("{}. This is the {} line.", 1, "first");

    // Positional arguments.
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Jack", "Boston", "code"
    );

    // Named arguments.
    println!("{name} likes to play {sport}", name = "Jack", sport = "Cricket");

    // Placeholder traits.
    println!("Binary: {:b}, Hexadecimal: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait.
    println!("{:?}", (12, true, "Hello"));      // Second argument is a tuple.

    // Basic arithmetic.
    println!("10 + 10 = {}", 10 + 10);
    println!("10 * 10 = {}", 10 * 10);
}
