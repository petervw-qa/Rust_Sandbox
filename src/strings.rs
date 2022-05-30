// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {

    // Primitive string example
    let hello = "Hello";

    // Get length
    println!("Length: {}", hello.len());

    // String (push capabilities - remember to declare as mutable)
    let mut new_hello = String::from("Hello ");

    // Push char
    new_hello.push('W');

    // Push string
    new_hello.push_str("orld!");

    // Capacity in Bytes
    println!("Capacity: {}", new_hello.capacity());

    // Check if empty
    println!("Is Empty: {}", new_hello.is_empty());

    // Contains
    println!("Contains 'World' {}", new_hello.contains("World"));

    // Replace
    println!("Replace: {}", new_hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in new_hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // Assertion Testing - won't print if successful, but will show failure in console
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}, {}", hello, new_hello);
}