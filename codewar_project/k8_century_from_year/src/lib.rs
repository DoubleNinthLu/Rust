// day 2 2021年6月7日
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(century(1705), 18);
        assert_eq!(century(1900), 19);
        assert_eq!(century(1601), 17);
        assert_eq!(century(2000), 20);
        assert_eq!(century(89), 1);
    }
}

fn century(year: u32) -> u32 {
    return (year + 99) / 100;
}
