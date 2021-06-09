// day 2 2021年6月7日
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }
}
fn order_old(sentence: &str) -> String {
    return match sentence.len() {
        0 => "".to_string(),
        _ => {
            let mut order_map = std::collections::HashMap::new();
            for x in sentence.split(" ") {
                for c in x.chars() {
                    if c >= '1' && c <= '9' {
                        order_map.insert(c.to_digit(10).unwrap(), x);
                        break;
                    }
                }
            }
            let mut res_temp = Vec::new();
            for x in 1u32..=order_map.len() as u32 {
                res_temp.push(*order_map.get(&x).unwrap());
            }
            res_temp.join(" ")
        }
    };
}

fn order(sentence: &str) -> String {
    let mut string_vec: Vec<String> = sentence.split_whitespace().map(String::from).collect();
    string_vec.sort_by_key(|string| string.chars().find(|c| c.is_digit(10)).unwrap());
    string_vec.join(" ")
}
