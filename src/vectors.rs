// Vectors are re-sizeable arrays.
// Vectors use more memory as compared to arrays.

use std::mem;

pub fn run() {
    let mut nums: Vec <i32> = vec![1, 2, 3, 4];

    println!("{:?}", nums);                     // Print the entire array.
    println!("Value at index 0: {}", nums[0]);

    // Add on to vector.
    nums.push(40);
    nums.push(20);
    nums.push(40);

    // Re-assign values.
    nums[2] = 10;

    println!("{:?}", nums);                     // Print the entire array.

    // Pop the last value.
    nums.pop();

    println!("{:?}", nums);                     // Print the entire array.

    // Loop through values in a vector.
    for x in nums.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate.
    for x in nums.iter_mut() {
        // *x += 2;             // add 2 to each value.
        *x *= 2;
    }

    println!("{:?}", nums);                     // Print the entire array.

    // Get length of an vector.
    println!("Length of vector nums: {}", nums.len());
    println!("Memory occupied by array nums: {} bytes", mem::size_of_val(&nums));

    // Get slice.
    let slice: &[i32] = &nums[0..3];
    println!("Slice: {:?}", slice);
}
