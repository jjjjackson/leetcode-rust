struct Solution {}

// https://leetcode.com/problems/maximum-sum-circular-subarray/solutions/3066058/c-easy-solution-with-explaination-in-o-n-time-complexity/
// 理論蠻特別的
impl Solution {
	pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
		let sum: i32 = nums.iter().sum();
		let mut dp = vec![[0; 2]; nums.len()];
		let mut result = nums[0];
		dp[0][0] = nums[0];

		(1..nums.len()).for_each(|i| {
			dp[i][0] = nums[i].max(dp[i - 1][0] + nums[i]);
			dp[i][1] = nums[i].min(dp[i - 1][1] + nums[i]);
			result = result.max(dp[i][0].max(sum - dp[i][1]));
		});

		result
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(vec![1,-2,3,-2],3)]
	#[test_case(vec![1,2,-2,4,3],10)]
	#[test_case(vec![-2,-3,-2],-2)]
	#[test_case(vec![-2],-2)]
	#[test_case(vec![3,-1,2,-1],3)]
	fn cases(nums: Vec<i32>, expected: i32) {
		assert_eq!(super::Solution::max_subarray_sum_circular(nums), expected);
	}
}
