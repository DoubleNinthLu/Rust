fn five() -> i32 {
    4
}

fn main() {
    let x = five();
    println!("{}", x);
    let y = plus_one(x);
    println!("{}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
