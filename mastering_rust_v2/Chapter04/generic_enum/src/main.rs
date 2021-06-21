#[derive(Debug)]
enum Transmission<T> {
    Signal(T),
    NoSignal,
}

fn main() {
    let a = Transmission::<u32>::Signal(5);
    println!("{:?} Hello, world!", a);
}
