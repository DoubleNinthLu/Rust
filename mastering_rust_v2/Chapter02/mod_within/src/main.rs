mod food {
    pub struct Cake;
    struct Smoothie;
    struct Pizza;
}

use food::*;

fn main() {
    let eatable = Cake;
}
