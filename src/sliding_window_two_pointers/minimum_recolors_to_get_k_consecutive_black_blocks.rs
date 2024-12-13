pub struct Solution;

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let k = k as usize;
        let lens = blocks.len();
        let mut num = vec![0; lens];
        for (pos, ch) in blocks.bytes().enumerate() {
            if ch == b'W' {
                num[pos] = 1;
            }
        }
        let mut sum = 0;
        for i in 0..k {
            sum += num[i];
        }
        let mut min = sum;
        for i in k..lens {
            sum -= num[i - k];
            sum += num[i];
            min = min.min(sum);
        }
        min
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_minimum_recolors() {
        let blocks = String::from("WBWBBBW");
        let k = 2;
        let min = Solution::minimum_recolors(blocks, k);
        assert_eq!(min, 0, "Excepted min to be 0, but got {}", min);
    }
}
