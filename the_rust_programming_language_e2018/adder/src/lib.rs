#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_test() {
        assert_ne!(2 + 2, 4);
    }

    #[test]
    fn another_test() {
        panic!("error");
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, super::add_two(2));
    }
}

pub fn add_two(a: i32) -> i32 {
    println!("test");
    a + 2
}
