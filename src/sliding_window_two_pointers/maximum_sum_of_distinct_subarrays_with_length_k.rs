pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k: usize = k as usize;
        let lens = nums.len();
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut sum: i64 = 0;
        for i in 0..k {
            sum += nums[i] as i64;
            map.entry(nums[i])
                .and_modify(|value| *value = *value + 1)
                .or_insert(1);
        }
        let mut max_sum: i64;
        if map.len() == k {
            max_sum = sum;
        } else {
            max_sum = 0;
        }
        for i in k..lens {
            sum -= nums[i - k] as i64;
            map.entry(nums[i - k])
                .and_modify(|value| *value = *value - 1);
            sum += nums[i] as i64;
            map.entry(nums[i])
                .and_modify(|value| *value = *value + 1)
                .or_insert(1);
            let &num = map.get(&(nums[i - k])).unwrap();
            if num == 0 {
                map.remove(&nums[i - k]);
            }
            if map.len() == k {
                if max_sum < sum {
                    max_sum = sum;
                }
            }
        }

        max_sum
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_maxmum_subarray_sum() {
        let nums = vec![9, 9, 9, 1, 2, 3];
        let k = 3;
        let max_sum = Solution::maximum_subarray_sum(nums, k);

        assert_eq!(
            max_sum, 12,
            "Expected max_sum to be 12, but got {}",
            max_sum
        )
    }
}
