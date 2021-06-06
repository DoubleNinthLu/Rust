// day 1 2021年6月6日
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(duplicate_encode("din"), "(((");
        assert_eq!(duplicate_encode("recede"), "()()()");
        assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
        assert_eq!(duplicate_encode("(( @"), "))((");
    }
}
fn duplicate_encode(word: &str) -> String {
    let word = word.to_lowercase();
    let mut encode_map = std::collections::HashMap::new();

    for c in word.chars() {
        *encode_map.entry(c).or_insert(0) += 1;
    }

    word.chars()
        .map(|c| match *encode_map.get(&c).unwrap() {
            1 => "(",
            _ => ")",
        })
        .collect()
}
