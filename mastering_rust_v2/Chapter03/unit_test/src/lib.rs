#[cfg(test)]
mod tests {
    use super::*;

    fn sum_inputs_outputs() -> Vec<((i8, i8), i8)> {
        vec![((1, 1), 2), ((0, 0), 0), ((2, -2), 0)]
    }

    #[test]
    fn test_sums() {
        sum_inputs_outputs()
            .into_iter()
            .for_each(|((a, b), c)| assert_eq!(sum(a, b), c));
    }
}

fn sum(a: i8, b: i8) -> i8 {
    a + b
}
