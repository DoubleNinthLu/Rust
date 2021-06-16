// day6 2021年6月15日
#[cfg(test)]
mod tests {

    use super::*;
    fn testing(epsilon: f64, exp: (i32, f64)) -> () {
        assert_eq!(iter_pi(epsilon), exp)
    }

    #[test]
    fn tests_iter_pi() {
        testing(0.1, (10, 3.0418396189));
        testing(0.01, (100, 3.1315929036));
        testing(0.001, (1000, 3.1405926538));
        testing(0.0001, (10000, 3.1414926536));
        testing(0.00001, (100001, 3.1416026535));
        testing(0.000001, (1000001, 3.1415936536));
        testing(0.05, (20, 3.0916238067));
    }
}

fn rnd10(f: f64) -> f64 { (f * 1e10).round() / 1e10 }

fn iter_pi(epsilon: f64) -> (i32, f64) {
    let pi = std::f64::consts::PI;
    let mut numerator = 1;
    let mut denominator = 1;
    let mut approxPi: f64 = 0_f64;
    let mut iter = 0;
    fn diff(approxPi: f64, pi: f64) -> f64 {
        let d = pi - (approxPi*4_f64);
        return d.abs();
    }
    while diff(approxPi, pi) > epsilon {
        approxPi += (numerator as f64)/(denominator as f64);
        numerator *= -1;
        denominator += 2;
        iter += 1;
    }
    return (iter, rnd10(approxPi*4_f64));
}
