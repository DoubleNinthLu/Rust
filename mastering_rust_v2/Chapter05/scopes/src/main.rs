fn main() {
    let level_0_str = String::from("foo");
    {
        let level_1_number = 9;
        {
            let mut level_2_vector = vec![1, 2, 3];
            level_2_vector.push(level_1_number);
            let test = level_2_vector.pop().unwrap();
            println!("{} {}", level_1_number, test);
        }
        println!("{}", level_1_number);
    }
}
