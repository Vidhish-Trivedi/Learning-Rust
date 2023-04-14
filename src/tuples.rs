/*
    Tuples are used to group together values of different type.
    Maximum of 12 elements.
*/

pub fn run() {
    let person: (&str, &str, i8) = ("Jack", "Boston", 25);

    println!("{} is from {} and is {} years old.", person.0, person.1, person.2);
}
