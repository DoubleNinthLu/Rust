fn main() {
    let score = 201;
    increase_by(score, 30);
}

fn increase_by(mut val: u32, how_much: u32) {
    val += how_much;
    println!("{}", val);
}