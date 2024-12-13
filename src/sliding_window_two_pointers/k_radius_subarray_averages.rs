pub struct Solution;

impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let lens = nums.len();
        let mut result = vec![-1; lens];
        if (k * 2) as usize > lens {
            return result;
        }
        let mut sum: i64 = 0;
        let k = k as usize;
        for i in 0..2 * k {
            sum += nums[i] as i64;
        }
        for i in k..lens - k {
            sum += nums[i + k] as i64;
            result[i] = (sum / (2 * k + 1) as i64) as i32;
            sum -= nums[i - k] as i64;
        }
        result
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_get_averages() {
        let nums = vec![1, 11, 17, 21, 29];
        let k = 4;
        let result = Solution::get_averages(nums, k);
        assert_eq!(
            result,
            vec![-1, -1, -1, 5, 4, 4, -1, -1, -1],
            "Excepted result to be [-1, -1, -1, 5, 4, 4, -1, -1, -1], but got {:?}",
            result
        );
    }
}
