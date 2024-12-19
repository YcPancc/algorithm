pub struct Solution;

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k: usize = k as usize;
        let mut sum: i32 = card_points[card_points.len() - k..].iter().sum();

        let mut max = sum;
        for i in 0..k {
            sum += card_points[i];
            sum -= card_points[card_points.len() - k + i];
            max = max.max(sum)
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_max_score() {
        let card_points = vec![100, 40, 17, 9, 73, 75];
        let k = 3;
        let max_score = Solution::max_score(card_points, k);
        assert_eq!(
            max_score, 248,
            "Excepted max_score to be 248, but got {}",
            max_score
        );
    }
}
