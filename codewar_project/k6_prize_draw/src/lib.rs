//! Day 7 2021年6月19日

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(st: &str, we: Vec<i32>, n: usize, exp: &str) -> () {
        assert_eq!(rank(st, we, n), exp)
    }

    #[test]
    fn basics_rank() {
        testing(
            "Addison,Jayden,Sofia,Michael,Andrew,Lily,Benjamin",
            vec![4, 2, 1, 4, 3, 1, 2],
            4,
            "Benjamin",
        );
        testing(
            "Elijah,Chloe,Elizabeth,Matthew,Natalie,Jayden",
            vec![1, 3, 5, 5, 3, 6],
            2,
            "Matthew",
        );
        testing(
            "Aubrey,Olivai,Abigail,Chloe,Andrew,Elizabeth",
            vec![3, 1, 4, 4, 3, 2],
            4,
            "Abigail",
        );
        testing("Lagon,Lily", vec![1, 5], 2, "Lagon");
    }
}

fn rank(st: &str, we: Vec<i32>, n: usize) -> &str {
    if st.len() == 0 {
        return "No participants";
    }
    let names = st.split(',');
    let mut raffles = names
        .clone()
        .into_iter()
        .zip(we)
        .map(|(chars, we)| letter_values(chars) * we)
        .zip(names)
        .collect::<Vec<(i32, &str)>>();

    raffles.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
    raffles.sort_by(|&(a, _), &(b, _)| b.cmp(&a));
    let (_, winning_name) = raffles
        .into_iter()
        .nth(n - 1)
        .unwrap_or((0, "Not enough participants"));
    winning_name
}

fn letter_values(value: &str) -> i32 {
    let charactor_total: i32 = value
        .chars()
        .map(|c| (c.to_digit(36).unwrap_or(0) - 9) as i32)
        .sum();
    charactor_total + value.len() as i32
}
