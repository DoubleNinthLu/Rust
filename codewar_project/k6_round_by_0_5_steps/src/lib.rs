//! Day 7 2021年6月19日
#[cfg(test)]
mod tests {
    use super::solution;
    #[test]
    fn sample_tests() {
        assert_eq!(solution(4.2), 4.0);
        assert_eq!(solution(4.4), 4.5);
        assert_eq!(solution(4.6), 4.5);
        assert_eq!(solution(4.8), 5.0);
    }
}

fn origin_solution(n: f64) -> f64 {
    let round = n.round();
    if round <= n {
        // n - n.5
        close_num(round, n, round + 0.5_f64)
    } else {
        close_num(round - 0.5_f64, n, round)
    }
}

fn close_num(a: f64, b: f64, c: f64) -> f64 {
    if (b - a) < (c - b) {
        a
    } else {
        c
    }
}

fn solution(n: f64) -> f64 {
    (2.0 * n).round() / 2.0
}
