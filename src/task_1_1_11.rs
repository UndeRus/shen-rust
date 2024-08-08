use crate::task_1_1_8::fac;

fn facsum(n: u64) -> f64 {
    let mut r: f64 = 0.0;
    for i in 0..=n {
        r += 1.0 / fac(i) as f64; 
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
    }
}
