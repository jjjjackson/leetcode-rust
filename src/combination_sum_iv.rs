use crate::solution::Solution;

impl Solution {
	pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
		let mut dp = vec![0i32; (target + 1) as usize];
		dp[0] = 1;

		for i in 1..(target + 1) {
			nums.iter().for_each(|n| {
				if n <= &i {
					dp[i as usize] = dp[i as usize] + dp[(i - n) as usize];
				}
			});
		}
		dp[target as usize]
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(vec![1,2,3], 4, 7)]
	fn success_cases(nums: Vec<i32>, target: i32, expected: i32) {
		assert_eq!(super::Solution::combination_sum4(nums, target), expected);
	}
}
