fn main() {
    let doubler = |x| x * 2;
    let value = 5;
    let twice = (doubler(value) as i32).to_string();
    println!("{} {}", value, twice);
    let big_closure = |b, c| {
        let z = b + c;
        twice
    };
    let some_number = big_closure(1, 2);
    println!("{}", some_number);
}
