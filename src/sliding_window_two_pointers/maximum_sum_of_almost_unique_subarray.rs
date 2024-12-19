pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_sum(nums: Vec<i32>, m: i32, k: i32) -> i64 {
        let m: usize = m as usize;
        let k: usize = k as usize;
        let lens: usize = nums.len();
        if lens < m {
            return 0 as i64;
        }
        let mut map: HashMap<i64, i32> = HashMap::new();
        let mut sum: i64 = 0;
        for i in 0..k {
            sum += nums[i] as i64;
            map.entry(nums[i] as i64)
                .and_modify(|value| *value = *value + 1)
                .or_insert(1);
        }
        let mut max: i64;
        if map.len() >= m {
            max = sum;
        } else {
            max = 0;
        }
        for i in k..lens {
            sum -= nums[i - k] as i64;
            map.entry(nums[i - k] as i64)
                .and_modify(|value| *value = *value - 1);
            let &num = map.get(&(nums[i - k] as i64)).unwrap();
            if num == 0 {
                map.remove(&(nums[i - k] as i64));
            }
            map.entry(nums[i] as i64)
                .and_modify(|value| *value = *value + 1)
                .or_insert(1);
            sum += nums[i] as i64;
            if map.len() >= m {
                if max < sum {
                    max = sum;
                }
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_max_sum() {
        let nums = vec![1, 1, 1, 3];
        let m = 2;
        let k = 2;

        let max_sum = Solution::max_sum(nums, m, k);

        assert_eq!(
            max_sum, 4 as i64,
            "Expected max_sum to be 4, but got {}",
            max_sum
        )
    }
}
