#[derive(Debug)]
struct Number<'a> {
    num: &'a u8,
}

impl<'a> Number<'a> {
    fn get_num(&self) -> &u8 {
        self.num
    }
    fn set_num(&mut self, new_number: &'a u8) {
        self.num = new_number;
    }
}

fn main() {
    let mut a = Number { num: &32 };
    // println!("{}", a.set_num(&31));
}
