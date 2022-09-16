// https://leetcode.com/problems/maximum-score-from-performing-multiplication-operations/submissions/
use crate::solutions::Solution;

impl Solution {
	fn maximum_score_helper(
		nums: &Vec<i32>,
		multipliers: &Vec<i32>,
		i: i32,
		n: i32,
		j: i32,
		dp: &mut Vec<Vec<i32>>,
	) -> i32 {
		if j == multipliers.len() as i32 {
			0
		} else if dp[i as usize][j as usize] != i32::MIN {
			dp[i as usize][j as usize]
		} else {
			let left = Self::maximum_score_helper(
				nums,
				multipliers,
				i + 1,
				n,
				j + 1,
				dp,
			) + nums[i as usize] * multipliers[j as usize];
			let right =
				Self::maximum_score_helper(nums, multipliers, i, n, j + 1, dp)
					+ nums[((n - 1) - (j - i)) as usize]
						* multipliers[j as usize];

			dp[i as usize][j as usize] = left.max(right);
			dp[i as usize][j as usize]
		}
	}

	pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
		let mut dp =
			vec![vec![i32::MIN; multipliers.len() + 1]; nums.len() + 1];
		Self::maximum_score_helper(
			&nums,
			&multipliers,
			0,
			nums.len() as i32,
			0,
			&mut dp,
		)
	}
}
