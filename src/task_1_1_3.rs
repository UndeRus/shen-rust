fn pow(a: i64, n: i64) -> i64 {
    let mut curr_pow = 0;
    let mut result = 1;
    while curr_pow < n {
        result = result * a;
        curr_pow += 1;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1_1_3() {
        assert_eq!(pow(2, 3), 8);
        assert_eq!(pow(-2, 3), -8);
        assert_eq!(pow(100, 0), 1);
    }
}
