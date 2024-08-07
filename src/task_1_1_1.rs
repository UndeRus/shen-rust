fn swap(a: i32, b: i32) -> (i32, i32) {
   let mut a = a;
   let mut b = b;
   let t = a;
   a = b;
   b = t;

   (a, b)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1_1_1() {
        let a = 3;
        let b = 6;

        let (new_a, new_b) = swap(a,b);

        assert_eq!(new_a, b);
        assert_eq!(new_b, a);
    }
}
