use crate::solutions::Solution;
use std::convert::TryInto;

// 1220. Count Vowels Permutation
// https://leetcode.com/problems/count-vowels-permutation/
// 思路不難，難在數字型態轉換
impl Solution {
	pub fn count_vowel_permutation(n: i32) -> i32 {
		let n: usize = n.try_into().unwrap();
		let m = 1000000007;
		// assume (a,e,i,o,u) = (0,1,2,3,4)
		let mut dp = vec![vec![0u32; 5]; n];

		for i in 0..5 {
			dp[0][i] = 1;
		}

		for i in 1..n {
			dp[i][0] = dp[i - 1][1] % m; // f(i-1)(e)
			dp[i][1] = (dp[i - 1][0] + dp[i - 1][2]) % m; // f(i-1)(a) + f(i-1)(i)
			dp[i][2] = (dp[i - 1][0] + dp[i - 1][1] + dp[i - 1][3] + dp[i - 1][4]) % m; // f(i-1)(a) + f(i-1)(e) + f(i-1)(o) + f(i-1)(u)
			dp[i][3] = (dp[i - 1][2] + dp[i - 1][4]) % m; // f(i-1)(i) + f(i-1)(u)
			dp[i][4] = dp[i - 1][0] % m; // f(i-1)(a)
		}

		let mut result = 0;
		for i in 0..5 {
			result = (result + dp[n - 1][i]) % m;
		}

		result as i32
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(10, 1739)]
	#[test_case(144, 18208803)]
	#[test_case(70, 581980397)]
	#[test_case(30, 761083377)]
	fn success_case(n: i32, expected: i32) {
		assert_eq!(super::Solution::count_vowel_permutation(n), expected);
	}
}
