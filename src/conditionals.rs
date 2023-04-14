pub fn run() {
    let age: i8 = 21;
    let check_id: bool = false;
    let is_verified: bool = true;

    // if-else.
    if (age >= 21 && check_id) || is_verified {
        println!("Can Vote!");
    } else if age < 21 && check_id {
        println!("Can not vote.");
    } else {
        println!("Tell me your age.");
    }

    // There is no ternary operator in Rust, but there is a shorthand for "if".
    let is_old = if age >= 30 { true } else { false };
    println!("{}", is_old);
}
