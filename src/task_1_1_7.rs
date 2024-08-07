fn divmod(a: u64, b: u64) -> (u64, u64) {
    let mut div = 0;
    let mut modv = a;
    while modv >= b {
        div += 1;
        modv -= b;
    }
    (div, modv)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1_1_7() {
        assert_eq!(divmod(10, 3), (3, 1));
        assert_eq!(divmod(15, 4), (3, 3));
    }
}
