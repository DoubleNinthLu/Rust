fn main() {
    let target = "world";
    let mut greeting = "hello";
    println!("{}, {}", greeting, target);
    greeting = "How are you doing";
    // cannot assign twice to immutable variable
    // target = "mate";
    println!("{}, {}", greeting, target);
}
