pub fn run() {
    // Print to Console
    println!("Hello from the print.rs file!");

    // Basic Formatting
    println!("Number: {}", 1);

    println!("{} is from {}", "Peter", "London");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Peter", "London", "code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "Peter",
        activity = "Football"
    );

    // Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}
