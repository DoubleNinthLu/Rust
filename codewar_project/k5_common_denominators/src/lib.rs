// day6 2021年6月13日
#[cfg(test)]
mod tests {
    use super::*;

    fn testing(l: Vec<(i64, i64)>, exp: Vec<(i64, i64)>) -> () {
        assert_eq!(convert_fracts(l), exp)
    }

    #[test]
    fn basics_convert_fracts() {
        testing(
            vec![(69, 130), (87, 1310), (3, 4)],
            vec![(18078, 34060), (2262, 34060), (25545, 34060)],
        );
        testing(
            vec![(690, 1300), (87, 1310), (30, 40)],
            vec![(18078, 34060), (2262, 34060), (25545, 34060)],
        );
    }
}

fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    if l.len() == 0 {
        return l;
    }

    let temp_vec = l.iter().map(|v| simplify(v.0, v.1)).collect::<Vec<_>>();
    let mut lcmt: Vec<i64> = temp_vec.to_vec().into_iter().map(|(_, v)| v).collect();

    loop {
        if lcmt.len() <= 1 {
            break;
        }
        lcmt = lcmt
            .chunks(2)
            .map(|temp_vec| match temp_vec.len() {
                2 => lcm(*temp_vec.get(0).unwrap(), *temp_vec.get(1).unwrap()),
                _ => *temp_vec.get(0).unwrap(),
            })
            .collect::<Vec<_>>();
    }

    let temp: i64 = *lcmt.get(0).unwrap();
    temp_vec
        .into_iter()
        .map(|v| (temp * v.0 / v.1, temp))
        .collect()
}

fn simplify(a: i64, b: i64) -> (i64, i64) {
    (a / gcd(a, b), b / gcd(a, b))
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut temp: i64;
    let mut a = a;
    let mut b = b;
    if a < b {
        temp = a;
        a = b;
        b = temp;
    }
    while b != 0 {
        temp = a % b;
        a = b;
        b = temp;
    }
    a
}
