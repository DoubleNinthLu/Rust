// day 1 2021年6月6日
fn find_average(slice: &[f64]) -> f64 {
    match slice.len() {
        0 => 0.0f64,
        n => slice.iter().sum::<f64>() / n as f64,
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
