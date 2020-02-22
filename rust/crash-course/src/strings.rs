// Primitive str = Immutable fixed-length string in memory
// String = growable heap-allocated data structure - Use when you need to
// modify or own string data

pub fn run() {
    let mut hello = String::from("Hello ");

    // get length
    println!("Length: {}", hello.len());

    // push a char
    hello.push('W');

    // push string
    hello.push_str("orl!");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // is it empty
    println!("Is Empty: {}", hello.is_empty());

    // contains
    println!("Contains 'World' {}", hello.contains("World"));

    // replace
    println!("Replace: {}", hello.replace("Worl", "There"));

    // loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", hello);
}
