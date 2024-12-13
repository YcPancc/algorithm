pub struct Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut sum = 0;
        let mut num = 0;
        let k: usize = k as usize;
        for i in 0..k {
            sum += arr[i];
        }
        if sum / k as i32 >= threshold {
            num += 1;
        }
        for i in k..arr.len() {
            sum -= arr[i - k];
            sum += arr[i];
            if sum / k as i32 >= threshold {
                num += 1;
            }
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_of_subarrays() {
        let arr = vec![2, 2, 2, 2, 5, 5, 5, 8];
        let k = 3;
        let threshold = 4;
        let num = Solution::num_of_subarrays(arr, k, threshold);
        assert_eq!(num, 3, "Expected num to be 3, but got {}", num);
    }
}
