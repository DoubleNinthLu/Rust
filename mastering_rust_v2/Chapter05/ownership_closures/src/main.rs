fn main() {
    let a = Foo;
    let closure = move || {
        let b = a;
    };
    println!("{:?}", a);
}

#[derive(Copy, Clone, Debug)]
struct Foo;

