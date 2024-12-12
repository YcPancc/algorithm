pub struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut max: i32;
        let k = k as usize;
        let mut sum = 0;
        for i in 0..k {
            sum += nums[i];
        }
        max = sum;
        for i in k..nums.len() {
            sum += nums[i];
            sum -= nums[i - k];
            max = max.max(sum);
        }
        max as f64 / k as f64
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_find_max_average() {
        let nums = vec![
            8860, -853, 6534, 4477, -4589, 8646, -6155, -5577, -1656, -5779, -2619, -8604, -1358,
            -8009, 4983, 7063, 3104, -1560, 4080, 2763, 5616, -2375, 2848, 1394, -7173, -5225,
            -8244, -809, 8025, -4072, -4391, -9579, 1407, 6700, 2421, -6685, 5481, -1732, -8892,
            -6645, 3077, 3287, -4149, 8701, -4393, -9070, -1777, 2237, -3253, -506, -4931, -7366,
            -8132, 5406, -6300, -275, -1908, 67, 3569, 1433, -7262, -437, 8303, 4498, -379, 3054,
            -6285, 4203, 6908, 4433, 3077, 2288, 9733, -8067, 3007, 9725, 9669, 1362, -2561, -4225,
            5442, -9006, -429, 160, -9234, -4444, 3586, -5711, -9506, -79, -4418, -4348, -5891,
        ];
        let k = 93;
        let max_average = Solution::find_max_average(nums, k);
        assert_eq!(
            max_average, -594.58065,
            "Excepted max_average to be -594.58065, but got {}",
            max_average
        )
    }
}
