// day5 2021年6月11日
#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(arr: Vec<&str>, exp: &str) -> () {
        println!("arr: {:?}", arr);
        let ans = part_list(arr);
        println!("actual:\n{}", ans);
        println!("expect:\n{}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basis_tests() {
        dotest(vec!["I", "wish", "I", "hadn't", "come"],
                "(I, wish I hadn't come)(I wish, I hadn't come)(I wish I, hadn't come)(I wish I hadn't, come)");
        dotest(
            vec!["cdIw", "tzIy", "xDu", "rThG"],
            "(cdIw, tzIy xDu rThG)(cdIw tzIy, xDu rThG)(cdIw tzIy xDu, rThG)",
        );
    }
}

fn part_list(arr: Vec<&str>) -> String {
    let mut res = String::new();
    for i in 0..(arr.len() - 1) {
        res.push_str(&format!(
            "({}, {})",
            arr[..=i].join(" "),
            arr[(i + 1)..].join(" ")
        ))
    }
    res
}
