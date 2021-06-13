// day3 2021年6月9日
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(first_non_consecutive(&vec![1, 2, 3, 4, 6, 7, 8]), Some(6));
        assert_eq!(first_non_consecutive(&vec![1, 2, 3, 4, 5, 6, 7, 8]), None);
        assert_eq!(first_non_consecutive(&vec![4, 6, 7, 8, 9, 11]), Some(6));
        assert_eq!(first_non_consecutive(&vec![4, 5, 6, 7, 8, 9, 11]), Some(11));
        assert_eq!(first_non_consecutive(&vec![31, 32]), None);
        assert_eq!(first_non_consecutive(&vec![-3, -2, 0, 1]), Some(0));
        assert_eq!(first_non_consecutive(&vec![-5, -4, -3, -1]), Some(-1));
    }
}

fn first_non_consecutive_origin(arr: &Vec<i32>) -> Option<i32> {
    let dis = arr.get(0).unwrap() - 0;
    match arr
        .into_iter()
        .enumerate()
        .map(|(idx, v)| (idx, (v - idx as i32)))
        .filter(|(_, v)| *v != dis)
        .next()
    {
        Some(v) => Some(v.1 + v.0 as i32),
        None => None,
    }
}

fn first_non_consecutive(arr: &Vec<i32>) -> Option<i32> {
    arr.windows(2).find(|t| t[0] + 1 != t[1]).map(|t| t[1])
}
