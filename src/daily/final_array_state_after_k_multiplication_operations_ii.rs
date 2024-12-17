pub struct Solution;

#[derive(PartialEq, Eq)]
pub struct Num {
    val: i128,
    pos: usize,
    times: i128,
}

use std::cmp::Ordering;
use std::collections::BinaryHeap;
impl Ord for Num {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.times.cmp(&self.times) {
            Ordering::Equal => match other.val.cmp(&self.val) {
                Ordering::Equal => other.pos.cmp(&self.pos),
                ordering => ordering,
            },
            ordering => ordering,
        }
    }
}

impl PartialOrd for Num {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        if multiplier == 1 {
            return nums;
        }
        let div_num: i128 = 1000_000_007;

        let mut minheap = BinaryHeap::new();
        for (pos, &num) in nums.iter().enumerate() {
            if (num as i128) < div_num {
                minheap.push(Num {
                    val: num as i128,
                    pos: pos,
                    times: 0 as i128,
                });
            } else {
                minheap.push(Num {
                    val: (num as i128) % div_num,
                    pos: pos,
                    times: (num as i128) / div_num,
                });
            }
        }
        let k = k as usize;
        let multiplier = multiplier as i128;
        for _ in 0..k {
            let mut min = minheap.pop().unwrap();
            min.val *= multiplier;
            min.times *= multiplier as i128;
            if min.val >= div_num {
                min.times += (min.val / div_num) as i128;
                min.val %= div_num;
            }
            minheap.push(min);
        }
        let mut result = vec![0; nums.len()];

        for num in minheap.into_iter() {
            result[num.pos] = (num.val % div_num) as i32;
        }

        result
    }
}

#[cfg(test)]

mod tests {
    use super::Solution;

    #[test]
    fn test_get_final_state() {
        let nums: Vec<i32> = vec![161209470];
        let k: i32 = 56851412;
        let multiplier: i32 = 39846;
        let result: Vec<i32> = Solution::get_final_state(nums, k, multiplier);
        assert_eq!(
            result,
            [198168519],
            "Expected result to be [198168519], but got {:?}",
            result
        );
    }
}
