// day4 2021年6月10日

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }
}

fn solution(s: &str) -> String {
    s.chars().map(|c| match c {
        'A'..='Z' => format!(" {}", c),
        _ => c.to_string(),
    }).collect()
}