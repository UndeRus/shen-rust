fn fib(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut k = n;
    let (mut a0, mut a1, mut b0, mut b1)  = (1, 1, 1, 0);
    let (mut aa0, mut aa1, mut bb0, mut bb1)  = (1, 1, 1, 0);
    while k != 0 {
        if k % 2 == 0 {
            k = k / 2;
            let (ta0, ta1, tb0, tb1) = mul2x2(a0, a1, b0, b1, a0, a1, b0, b1);
            a0 = ta0;
            a1 = ta1;
            b0 = tb0;
            b1 = tb1;
        } else {
            k = k - 1;
            let (ta0, ta1, tb0, tb1) = mul2x2(aa0, aa1, bb0, bb1, a0, a1, b0, b1);
            aa0 = ta0;
            aa1 = ta1;
            bb0 = tb0;
            bb1 = tb1;
        }
    }

    a1
}


fn mul2x2(a0: u64, a1: u64, b0: u64, b1: u64, aa0: u64, aa1: u64, bb0: u64, bb1: u64) -> (u64, u64, u64, u64) {
    let aaa0 = a0 * aa0 + a1 * bb0;
    let aaa1 = a0 * aa1 + a1 * bb1;
    let bbb0 = b0 * aa0 + b1 * bb0;
    let bbb1 = b0 * aa1 + b1 * bb1;

    (aaa0, aaa1, bbb0, bbb1)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1_1_10() {
        assert_eq!(mul2x2(1, 1, 1, 0, 1, 1, 1, 0), (2, 1, 1, 1));
        assert_eq!(mul2x2(5, 15, 8, 3, 3, 25, 9, 1), (150, 140, 51, 203));

        assert_eq!(fib(1), 1);
        assert_eq!(fib(0), 0);
        assert_eq!(fib(9), 21);
    }
}
