fn sum(a: u64, b: u64) -> u64 {
    let mut k = 0;
    let mut result = a;
    while k != b {
        result += 1;
        k += 1;
    }

    result

    //a1 = a2
    //a = N
    //a1 = a2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1_1_6() {
        assert_eq!(sum(2, 3), 5);
        assert_eq!(sum(7, 3), 10);
    }
}
