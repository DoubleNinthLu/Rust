fn main() {
    let a: &str = "Hello trait object";
    show_me(&a);
}

use std::fmt::Display;

fn show_me(item: &dyn Display) {
    println!("{}", item);
}
