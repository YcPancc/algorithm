pub struct Solution;

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let lens = code.len();
        let mut result = vec![0; lens];
        if k == 0 {
            return result;
        } else if k > 0 {
            let mut sum = 0;
            for i in 1..=k as usize {
                sum += code[i];
            }
            result[0] = sum;
            for i in 1..lens {
                sum -= code[i];
                let pos = (i + k as usize) % lens;
                sum += code[pos];
                result[i] = sum;
            }
        } else {
            let mut pos: i32 = 0;
            let mut sum = 0;
            let k = k.abs();
            for _ in 0..k {
                pos = pos - 1;
                if pos < 0 {
                    pos += lens as i32;
                }
                sum += code[pos as usize];
            }
            result[0] = sum;
            for i in 1..lens {
                sum += code[i - 1];
                let mut pos = i as i32 - k - 1;
                if pos < 0 {
                    pos = pos + lens as i32;
                }
                sum -= code[pos as usize];
                result[i] = sum;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_decrypt() {
        let code = vec![5, 2, 2, 3, 1];
        let k = 3;
        let pw = Solution::decrypt(code, k);
        assert_eq!(
            pw,
            vec![7, 6, 9, 8, 9],
            "Expected pw to be [7, 6, 9, 8, 9], but got {:?}",
            pw
        );
    }
}
