pub struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut max = 0;
        let mut vowel = 0;
        let k = k as usize;
        for (i, &ch) in s.iter().enumerate() {
            if ch == b'a' || ch == b'e' || ch == b'i' || ch == b'o' || ch == b'u' {
                vowel += 1;
            }
            if i < k - 1 {
                continue;
            }
            max = max.max(vowel);
            let out = s[i + 1 - k];
            if out == b'a' || out == b'e' || out == b'i' || out == b'o' || out == b'u' {
                vowel -= 1;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_vowels() {
        let s = String::from("abciiidef");
        let k = 3;
        let max_vowels = Solution::max_vowels(s, k);
        assert_eq!(
            max_vowels, 3,
            "Expected max_vowels to be 3, but got {}",
            max_vowels
        );
    }
}
