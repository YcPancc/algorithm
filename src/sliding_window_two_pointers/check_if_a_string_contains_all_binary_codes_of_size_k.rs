use std::collections::HashSet;
pub struct Solution;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        if k as usize <= s.len() {
            return false;
        }
        let mut set: HashSet<&str> = HashSet::new();
        let k: usize = k as usize;
        for i in 0..(s.len() - k + 1) {
            set.insert(&s[i..i + k]);
        }
        let mut num = 1;
        for _ in 0..k {
            num *= 2;
        }
        if set.len() != num {
            return false;
        }
        true
    }
}

#[cfg(test)]

mod tests {
    use super::Solution;

    #[test]
    fn test_has_all_codes() {
        let s: String = String::from("0000000001011100");
        let k: i32 = 4;

        let result = Solution::has_all_codes(s, k);

        assert_eq!(
            result, false,
            "Expected result to be true, but got {}",
            result
        );
    }
}
