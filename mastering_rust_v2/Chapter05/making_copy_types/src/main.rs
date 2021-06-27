fn main() {
    let a = Dummy;
    let b = a;
    println!("{:?} {:?}", a, b);
    println!("{} {}", a, b);

    let str_a = "1";
    let str_b = str_a;
    println!("{} {}", str_a, str_b);
}

use std::fmt::{Display, Result, Formatter};

#[derive(Copy, Clone, Debug)]
struct Dummy;

impl Display for Dummy {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", "Dummy")
    }
}

