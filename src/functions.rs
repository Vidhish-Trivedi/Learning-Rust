// Functions are used to store blocks of code for re-use.

pub fn run() {
    greeting("Hello", "Jack");

    // Bind function values to variables;
    let get_sum = my_add(5, 17);
    println!("Value from function: {}", get_sum);

    // Closure, allows us to use variables which are out-of-scope of the function.
    let n3: i32 = 40;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure sum: {}", add_nums(7, 10));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {} nice to meet you!", greet, name);
}

fn my_add(n1: i32, n2: i32) -> i32 {
    println!("Adding {} and {}", n1, n2);
    n1 + n2                             // Note that by not using a semi-colon here, Rust understands this to be a return statement.
}



