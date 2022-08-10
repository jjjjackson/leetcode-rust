// https://leetcode.com/problems/longest-increasing-subsequence/
use crate::solutions::Solution;
use std::cmp::max;

impl Solution {
	pub fn length_of_lis(nums: Vec<i32>) -> i32 {
		let mut dp = vec![0; (nums.len() + 1) as usize];
		let mut result = 0;

		for i in 1..(nums.len() + 1) {
			for j in 1..i {
				if nums[j - 1] < nums[i - 1] {
					dp[i] = max(dp[i], dp[j]);
				}
			}
			dp[i] += 1;
			result = max(result, dp[i]);
		}
		result
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(vec![10,9,2,5,3,7,101,18], 4)]
	#[test_case(vec![0,1,0,3,2,3], 4)]
	#[test_case(vec![7,7,7,7,7,7,7], 1)]
	fn test_success_case(nums: Vec<i32>, expected: i32) {
		assert_eq!(super::Solution::length_of_lis(nums), expected);
	}
}
