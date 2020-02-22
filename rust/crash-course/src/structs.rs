// Structs - used to create custom data types

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct T_Color(u8, u8, u8);

// Person
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 1,
        green: 1,
        blue: 1,
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    // Tuple array
    let mut t_c = T_Color(3, 2, 55);

    // Reasign
    t_c.0 = 30;

    println!("Color: {} {} {}", t_c.0, t_c.1, t_c.2);

    let mut p = Person::new("Rebecca", "Bott");
    println!("Person: {}", p.full_name());
    p.set_last_name("Sansbury");
    println!("Person: {}", p.full_name());

    println!("Person Tuple {:?}", p.to_tuple());
}
