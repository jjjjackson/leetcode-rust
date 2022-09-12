// https://leetcode.com/problems/bag-of-tokens/
// Solution from https://leetcode.com/problems/bag-of-tokens/discuss/2564435/LeetCode-The-Hard-Way-Explained-Line-By-Line

use crate::solutions::Solution;

impl Solution {
	pub fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
		let mut tokens = tokens;
		tokens.sort_by(|a, b| a.cmp(b));

		let mut power = power;
		let (mut l, mut r) = (0i32, tokens.len() as i32 - 1);
		let (mut cur_score, mut mx_score) = (0i32, 0i32);

		while l <= r {
			if tokens[l as usize] <= power {
				power -= tokens[l as usize];
				cur_score += 1;
				mx_score = mx_score.max(cur_score);
				l += 1;
			} else if cur_score >= 1 {
				power += tokens[r as usize];
				cur_score -= 1;
				r -= 1;
			} else {
				break;
			}
		}

		mx_score
	}
}
