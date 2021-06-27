fn main() {
    let foo = Foo(2048);

    // 不可变借用
    let bar = &foo;

    println!("foo is {:?}", foo);
    println!("bar is {:?}", bar);
}


#[derive(Debug)]
struct Foo(u32);
