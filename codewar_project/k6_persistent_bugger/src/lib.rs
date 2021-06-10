// day4 2021年6月10日
#[cfg(test)]
mod tests {

    #[test]
    fn sample_tests() {
        assert_eq!(super::persistence(39), 3);
        assert_eq!(super::persistence(4), 0);
        assert_eq!(super::persistence(25), 2);
        assert_eq!(super::persistence(999), 4);
    }
}

fn persistence(num: u64) -> u64 {
    let mut outer_num = num;
    let mut product;
    let mut counter = 0;
    while outer_num > 9 {
        product = 1;
        while outer_num != 0 {
            product *= outer_num % 10;
            outer_num /= 10;
        }
        outer_num = product;
        counter += 1;
    }
    counter
}
