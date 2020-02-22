// Variable hold primitaive data or references to data
// Immutable by default
// Rust is a bolck-scoped language

pub fn run() {
    let name = "Jacob";
    let mut age = 20;

    println!("My name is {} and I am {}", name, age);

    age = 21;

    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Jacob", 20);
    println!("My name is {} and I am {}", my_name, my_age);
}
