/*
    Arrays are fixed-length lists where elements are of the same data type.    
*/

use std::mem;

pub fn run() {
    let nums: [i32; 5] = [1, 3, 5, 7, 9];       // [d_type, len]
    
    println!("{:?}", nums);                     // Print the entire array.
    println!("Value at index 0: {}", nums[0]);

    let mut chars: [char; 6] = ['a', 'b', 'c', 'd', 'e', 'f'];
    println!("{:?}", chars);

    // Re-assign values.
    chars[2] = 'z';

    println!("{:?}", chars);

    // Get length of an array.
    println!("Length of array nums: {}", nums.len());
    
    // Arrays are stack allocated.
    // println!("Memory occupied by array nums: {} bytes", std::mem::size_of_val(&nums));      // std::mem --> standard library
    println!("Memory occupied by array nums: {} bytes", mem::size_of_val(&nums));

    // Slice an array.
    // let slice: &[i32] = &nums;
    // println!("slice: {:?}", slice);

    let slice: &[i32] = &nums[0..3];
    println!("slice: {:?}", slice);
}
