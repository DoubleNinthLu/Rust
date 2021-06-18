#![doc(html_logo_url = "https://d30y9cdsu7xlg0.cloudfront.net/png/411962-200.png")]
//! 这是一个逻辑门模拟器程序，用于演示单元测试和集成测试


/// 实现一个逻辑门 and，将两个8位作为输入，并返回一个8位
pub fn and(a: u8, b: u8) -> u8 {
    match (a, b) {
        (1, 1) => 1,
        _ => 0,
    }
}

/// 实现一个逻辑门 xor，将两个位作为输入，并返回一个位作为输出
pub fn xor(a: u8, b: u8) -> u8 {
    match (a, b) {
        (1, 0) | (0, 1) => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::{and, xor};

    #[test]
    fn test_and() {
        assert_eq!(1, and(1, 1));
        assert_eq!(0, and(0, 1));
        assert_eq!(0, and(1, 0));
        assert_eq!(0, and(0, 0));
    }

    #[test]
    fn test_xor() {
        assert_eq!(0, xor(1, 1));
        assert_eq!(1, xor(1, 0));
        assert_eq!(1, xor(0, 1));
        assert_eq!(0, xor(0, 0));
    }
}
