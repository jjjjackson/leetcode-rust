// https://leetcode.com/problems/stamping-the-sequence/
// Solution is from https://leetcode.com/problems/stamping-the-sequence/discuss/1135900/Rust-C%2B%2B%3A-Undo-The-Stamps-%2B-Explanation
use crate::solutions::Solution;

impl Solution {
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let (stamp, mut target) = (stamp.as_bytes(), target.into_bytes());
        let mut res = Vec::new();
        
        while target.iter().any(|&c| c != b'?') {
            let prev_res_len = res.len();
            
            for i in 0..target.len() - stamp.len() + 1 {
                let sub = &mut target[i..i+stamp.len()];
                
                if sub.iter().any(|&c| c != b'?')
                    && sub.iter().zip(stamp.iter()).all(|(&wc, &sc)| wc == sc || wc == b'?')
                {
                    sub.iter_mut().for_each(|c| *c = b'?');
                    res.push(i as _)
                }
            }
            
            if res.len() ==prev_res_len {
                return Vec::new();
            }
        }
        
        res.reverse();
        res
    }
}


#[cfg(test)]
mod tests{
    use test_case::test_case;

    #[test_case("abc", "ababc", vec![0,2])]
    #[test_case("abca", "aabcaca", vec![3,0,1])]
    fn success(stamp: &'static str, target: &'static str, expected: Vec<i32>) {
        let result = super::Solution::moves_to_stamp(stamp.to_string(), target.to_string());
        assert!(result.iter().all(|x| expected.contains(&x)));
    }
}