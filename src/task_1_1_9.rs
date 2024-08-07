fn fib(n: u64) -> u64 {
    let mut a0 = 0;
    let mut a1 = 1;
    if n == 0 {
        return a0;
    } else if n == 1 {
        return a1;
    }

    let mut n = n - 1;
    while n > 0 {
        let a2 = a0 + a1;
        a0 = a1;
        a1 = a2;
        n = n - 1;
    }
    a0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1_1_8() {
        assert_eq!(fib(1), 1);
        assert_eq!(fib(0), 0);
        assert_eq!(fib(9), 21);
    }
}
