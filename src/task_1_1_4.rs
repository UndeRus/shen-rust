fn pow(a: i64, n: i64) -> i64 {
    let mut k = n;
    let mut b = 1;
    let mut c = a;
    while k != 0 {
        if k % 2 == 0 {
            k = k / 2;
            c = c * c;
        } else {
            k = k - 1;
            b = b * c;
        }
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1_1_3() {
        assert_eq!(pow(2, 3), 8);
        assert_eq!(pow(2, 4), 16);
        assert_eq!(pow(-2, 3), -8);
        assert_eq!(pow(100, 0), 1);
    }
}
