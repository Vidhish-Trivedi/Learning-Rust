// Rust has an infinite loop. We need to break it when we need to.

pub fn run() {
    let mut cnt = 0;

    // Infinite Loop.
    loop {
        cnt += 1;
        println!("Number: {}", cnt);

        if cnt == 20 {
            break;
        }
    }

    println!();

    // While loop.
    while cnt <= 100 {
        cnt += 1;
        if cnt%15 == 0 {
            println!("FizzBuzz");
        }
        else if cnt%5 == 0 {
            println!("Buzz");
        }
        else if cnt%3 == 0 {
            println!("Fizz");
        }
        else{
            println!("Number: {}", cnt);
        }
    }

    println!();

    // For range loop.
    for x in 0..100 {               // [0,100)
        if x%15 == 0 {
            println!("FizzBuzz");
        }
        else if x%5 == 0 {
            println!("Buzz");
        }
        else if x%3 == 0 {
            println!("Fizz");
        }
        else{
            println!("Number: {}", x);
        }
    }
}
