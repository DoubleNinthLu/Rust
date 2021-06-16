#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    pub fn test_silly_loop() {
        super::silly_loop();
    }
}

pub fn silly_loop() {
    for _ in 1..1_000_000_000 {};
}
