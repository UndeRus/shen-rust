pub (crate) fn fac(a: u64) -> u64 {
    if a == 0 {
        return 1;
    }
    let mut k = a;
    let mut n = a;
    while k > 1 {
        k = k - 1;
        n = n * k;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1_1_8() {
        assert_eq!(fac(0), 1);
        assert_eq!(fac(1), 1);
        assert_eq!(fac(2), 2);
        assert_eq!(fac(3), 6);
        assert_eq!(fac(7), 5040);
    }
}
