fn main() {
    println!("Hello, world!");
}

#[test]
#[should_panic]
fn this_panics() {
    assert_eq!(1, 2);
}
