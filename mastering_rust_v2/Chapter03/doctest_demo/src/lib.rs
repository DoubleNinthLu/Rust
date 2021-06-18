//! This crate provides functionality for adding things
//! 
//! # Examples
//! 
//! ```
//! use doctest_demo::sum;
//! 
//! let work_a = 4;
//! let work_b = 34;
//! let totoal_work = sum(work_a, work_b);
//! ```

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


/// Sum tow arguments
/// 
/// # Examples
/// 
/// ```
/// assert_eq!(doctest_demo::sum(1, 1), 2);
/// ```
pub fn sum(a: i8, b: i8) -> i8 {
    a + b
}