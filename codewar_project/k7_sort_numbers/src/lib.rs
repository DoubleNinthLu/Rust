// day6 2021年6月13日
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(sort_numbers(&vec![1, 2, 3, 10, 5]), vec![1, 2, 3, 5, 10]);
        assert_eq!(sort_numbers(&vec![]), vec![]);
        assert_eq!(sort_numbers(&vec![20, 2, 10]), vec![2, 10, 20]);
        assert_eq!(sort_numbers(&vec![2, 20, 10]), vec![2, 10, 20]);
        assert_eq!(sort_numbers(&vec![2, 10, 20]), vec![2, 10, 20]);
    }
}

fn sort_numbers(arr: &Vec<i32>) -> Vec<i32> {
    // 1. 克隆
    // let mut res = arr.Clone();
    // 2. to_vec
    // let mut res = arr.to_vec();
    // 3. 手动
    let mut res = arr.into_iter().map(|v| *v).collect::<Vec<_>>();
    res.sort();
    res
}
