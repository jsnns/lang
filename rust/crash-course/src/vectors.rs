// Vectors - resizable arrays
use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(6);
    numbers.pop();

    println!("{:?}", numbers);

    // Get single val
    println!("Single Value: {}", numbers[0]);

    // Get vector length
    println!("Vector Length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    let mut mul = 2;
    for x in numbers.iter_mut() {
        *x *= mul;
        mul *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}
