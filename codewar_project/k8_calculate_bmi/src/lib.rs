// day 2 2021年6月7日
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(bmi(50, 1.80), "Underweight");
        assert_eq!(bmi(80, 1.80), "Normal");
        assert_eq!(bmi(90, 1.80), "Overweight");
        assert_eq!(bmi(110, 1.80), "Obese");
    }
}
fn bmi(weight: u32, height: f32) -> &'static str {
    return match weight as f32 / (height * height) {
        0.0..=18.5 => "Underweight",
        18.5..=25.0 => "Normal",
        25.0..=30.0 => "Overweight",
        _ => "Obese",
    };
}
