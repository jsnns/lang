use std::fmt::Debug;
use std::ops::Add;

trait Killer {
    fn yuck(&self);
}

struct Day {
    today: String,
}

impl Add for Day {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            today: format!("{} + {}", self.today, other.today),
        }
    }
}

impl Debug for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.today)
    }
}

trait Quack {
    fn quack(self);
}

impl Quack for bool {
    fn quack(self) {
        println!(
            "The duck {} quack.",
            if self { "should" } else { "shouldn't" }
        )
    }
}

impl Quack for f64 {
    fn quack(self) {
        let quack_str = "Booper is a princess";
        let rounded: i32 = self as i32;
        for _ in 0..rounded {
            println!("{}", quack_str)
        }

        let remainder = self - rounded as f64;
        let quack_str_len = quack_str.len();
        let len: usize = (quack_str_len as f64 * remainder) as usize;

        println!("{}", &quack_str[..len])
    }
}

#[derive(Debug)]
pub enum MyError {
    Server(u8),
    Oops(bool),
    Wow(String),
}

fn main() {
    println!("Hello from large");

    let new_day = Day {
        today: "Test".to_string(),
    };
    let yesterday = Day {
        today: "Yesterday".to_string(),
    };

    println!(
        "{:?}",
        new_day
            + yesterday
            + Day {
                today: "Nah".to_string()
            }
    );

    true.quack();

    3.5.quack();
}
