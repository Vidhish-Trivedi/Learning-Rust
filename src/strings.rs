/* Primitive str => immutable, fixed-length string somewhere in memory.
   String => Growable, heap-allocated data structure - Use when you need to modify or own string data.
*/

pub fn run() {
    let greeting = "hello ";     // Primitive str.

    let mut quote = String::from("A wise man once said: ");

    // Get length, works with primitive str and String type.
    println!("Length of str: {}", greeting.len());

    // push() [for chars] and push_str() methods, for String type.
    quote.push('A');                            // Push on a char.
    quote.push_str(" string");              // Push on a string.

    // Get capacity in bytes, for String type.
    println!("Capacity: {} bytes", quote.capacity());

    // Check if empty.
    println!("Is empty: {}", quote.is_empty());

    // Contains substr.
    println!("Contains 'string': {}", quote.contains("string"));
    println!("Contains 'String': {}", quote.contains("String"));

    // Replace.
    println!("Replace: {}", quote.replace("A string", "A line"));     // Returns a value, does not modify original string.

    // Loop through strings.
    for word in quote.split_whitespace() {
        println!("{}", word);
    }

    // Create string with specific capacity.
    let mut s = String::with_capacity(10); // Creates a new empty String with at least the specified capacity.

    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing.
    assert_eq!(2, s.len());             // If not passed --> error.
    assert_eq!(9, s.capacity());        // If not passed --> error.

    println!("Primitive str: {}\nString: {}", greeting, quote);
}
