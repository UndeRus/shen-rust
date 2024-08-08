fn facsum(a: u64) -> f64 {
    let mut r: f64 = 2.0;

    if a == 0 {
        return 1.0;
    }
    if a == 1 {
        return 2.0;
    }
    let mut k = a;
    let mut n = a;
    while k > 1 {
        k = k - 1;
        n = n * k;
        r += 1.0 / n as f64; 
    }

    r
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1_1_10() {
        assert_eq!(facsum(0), 1.0);
        assert_eq!(facsum(1), 2.0);
        assert_eq!(facsum(2), 2.5);
        assert_eq!(facsum(3), 2.333333333333333);
    }
}
