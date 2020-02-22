struct Person {
    name: std::string::String,
    activity: std::string::String,
}

pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // print an integer, with formatting stringd
    println!("Number: {}", 1);

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Jacob", "SC", "code"
    );

    // Named arguments
    println!(
        "{name} likes to play {activity}",
        name = "Jacob",
        activity = "Racing"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
