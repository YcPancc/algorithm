pub struct Solution;

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let lens: usize = customers.len();
        let minutes: usize = minutes as usize;
        let mut all_user: i32 = 0;
        let mut not_sat_user: i32 = 0;
        for i in 0..lens {
            all_user += customers[i];
            if grumpy[i] == 1 {
                not_sat_user += customers[i];
            }
        }
        let mut not_sat_ran_num: i32 = 0;
        let mut max_not_sat_ran_num: i32;
        for i in 0..minutes {
            if grumpy[i] == 1 {
                not_sat_ran_num += customers[i];
            }
        }
        max_not_sat_ran_num = not_sat_ran_num;
        for i in minutes..lens {
            if grumpy[i] == 1 {
                not_sat_ran_num += customers[i];
            }
            if grumpy[i - minutes] == 1 {
                not_sat_ran_num -= customers[i - minutes];
            }
            max_not_sat_ran_num = max_not_sat_ran_num.max(not_sat_ran_num);
        }
        all_user - not_sat_user + max_not_sat_ran_num
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_max_satisfied() {
        let customers: Vec<i32> = vec![1, 0, 1, 2, 1, 1, 7, 5];
        let grumpy: Vec<i32> = vec![0, 1, 0, 1, 0, 1, 0, 1];
        let minutes: i32 = 3;
        let max_sat: i32 = Solution::max_satisfied(customers, grumpy, minutes);
        assert_eq!(
            max_sat, 16,
            "Expected max_sat to be 16, but got {}",
            max_sat
        );
    }
}
