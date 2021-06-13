// day3 2021年6月9日
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(expanded_form(12), "10 + 2");
        assert_eq!(expanded_form(42), "40 + 2");
        assert_eq!(expanded_form(70304), "70000 + 300 + 4");
    }
}

fn expanded_form(n: u64) -> String {
    let temp = n.to_string();
    let len = temp.len();
    temp.chars()
        .enumerate()
        .map(|(i, c)| match c {
            '1'..='9' => format!("{}{}", c, "0".repeat(len - i - 1)),
            _ => "".to_string(),
        })
        .filter(|str_null| str_null != "")
        .collect::<Vec<_>>()
        .join(" + ")
}
