pub struct Solution;

impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut result = nums.clone();
        let mut min: i32;
        let mut pos: usize;
        for _ in 0..k {
            min = result[0];
            pos = 0;
            for (i, &val) in result.iter().enumerate() {
                if min > val {
                    min = val;
                    pos = i;
                }
            }
            result[pos] = result[pos] * multiplier;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_get_final_state() {
        let nums = vec![1, 3, 5];
        let k = 5;
        let multiplier = 3;
        let result = Solution::get_final_state(nums, k, multiplier);
        assert_eq!(
            result,
            vec![27, 9, 15],
            "Expected result to be [27, 9, 15], but got {:?}",
            result
        );
    }
}
