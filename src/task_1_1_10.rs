fn fib(n: u64) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1_1_10() {
        assert_eq!(fib(1), 1);
        assert_eq!(fib(0), 0);
        assert_eq!(fib(9), 21);
    }
}
