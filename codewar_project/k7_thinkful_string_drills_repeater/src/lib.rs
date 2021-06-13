// day5 2021年6月11日
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn basic_test() {
        assert_eq!(repeater("a", 5), "aaaaa");
        assert_eq!(repeater("Na", 16), "NaNaNaNaNaNaNaNaNaNaNaNaNaNaNaNa");
        assert_eq!(repeater("Wub ", 6), "Wub Wub Wub Wub Wub Wub ");
    }
}

fn repeater(string: &str, n: u32) -> String {
    let mut res = String::new();
    for _ in 0..n {
        res.push_str(string);
    }
    res
}
