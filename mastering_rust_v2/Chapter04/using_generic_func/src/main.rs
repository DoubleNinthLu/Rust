use std::str;

fn main() {
    let num_from_str = str::parse::<u8>("34").unwrap();
    println!("{} ", num_from_str);
}
