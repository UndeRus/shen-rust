fn mult(a: u64, b: u64) -> u64 {
    let mut result = 0;
    let mut k = b;
    while k != 0 {
        result += a;
        k -= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1_1_5() {
        assert_eq!(mult(2, 3), 6);
        assert_eq!(mult(7, 3), 21);
    }
}
