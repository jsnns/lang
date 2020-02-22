use std::option;

fn run() {
    let foo_ = Foo {};
    let op = option::Option::from(&foo_);
    f(op);

    match h(10) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    if let Err(e) = h(-10) {
        println!("Error from if let: {}", e)
    }

    println!("===");

    let foo_neg = foo(Option::from(-10));
    println!("Foo neg: {:?}", foo_neg);

    let foo_pos = foo(Option::from(10));
    println!("Foo pos: {:?}", foo_pos);

    let foo_none = foo(Option::from(None));
    println!("Foo none: {:?}", foo_none);

    ping_all(&[1, 2, 3, 4, 5]);

    let vec = vec![0, 1, 2, 3];

    let mut iter = (&vec).into_iter();
    loop {
        match iter.next() {
            Some(n) => println!("{}", n),
            None => break,
        }
    }
}

struct Foo {}

impl Foo {
    fn g(&self) {
        println!("Hello World!")
    }
}

fn f(ptr: option::Option<&Foo>) {
    match ptr {
        Some(ptr) => ptr.g(),
        None => {}
    }
}

fn h(i: i32) -> Result<i32, String> {
    match i {
        i if i >= 0 => Ok(i + 10),
        _ => Err(format!("Input to h < 0 found: {}", i)),
    }
}

fn foo(input: Option<i32>) -> Option<i32> {
    input.and_then(|x| if x < 0 { None } else { Some(x) })
}

fn bar(input: Option<i32>) -> Result<i32, String> {
    foo(input).ok_or("Damn!".to_string())
}

fn ping_all(foos: &[i32]) {
    foos.iter().for_each(|i| println!("This is number {}", i))
}
