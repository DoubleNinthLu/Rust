fn main() {
    println!("Hello, world!");
    Pancakes::hello_macro();
}
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;
