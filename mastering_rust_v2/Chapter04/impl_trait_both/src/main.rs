use std::fmt::Display;

fn surround_with_braces(val: impl Display) -> impl Display {
    // 额外的花括号是为了转义花括号自身
    let a = format!("{{{}}}", val);
    let b = format!("{{}}");
    println!("{:?}", b);
    a
}

fn main() {
    println!("{}", surround_with_braces("hello"));
}
