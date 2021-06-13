// day3   2021年6月9日
#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(p0: i32, percent: f64, aug: i32, p: i32, exp: i32) -> () {
        println!("p0: {:?};", p0);
        println!("percent: {:?};", percent);
        println!("aug: {:?};", aug);
        println!("p: {:?};", p);
        let ans = nb_year(p0, percent, aug, p);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(1500, 5.0, 100, 5000, 15);
        dotest(1500000, 2.5, 10000, 2000000, 10);
        dotest(1500000, 0.25, 1000, 2000000, 94);
        dotest(909305, 0.38, 4546, 1927489, 105);
    }
}

fn nb_year(p0: i32, percent: f64, aug: i32, p: i32) -> i32 {
    let mut new_percent = percent;
    let mut new_p0: f64 = p0 as f64;

    new_percent /= 100.0;

    let mut count = 0;
    while (new_p0 as i32) < p {
        new_p0 = new_p0 * (1.0 + new_percent) + aug as f64;
        new_p0 = (new_p0 as i32) as f64;
        count += 1;
    }
    count
}
