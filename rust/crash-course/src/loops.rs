// Loops - used to iterate until a condition is met

pub fn run() {
    let mut count = 0;

    // Infinite
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 20 {
            break;
        }
    }

    count = 1;
    // While Loop (FizzBuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("Fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", count);
        }

        count += 1;
    }

    // for range
    for x in 0..100 {
        println!("{}", x);
    }
}
