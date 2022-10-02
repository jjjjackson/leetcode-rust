use crate::solutions::Solution;

impl Solution {
	pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
		let (n, k, target) = (n as usize, k as usize, target as usize);
		let mut dp = vec![vec![usize::MAX; target + 1]; n + 1];

		Self::num_rolls_to_target_helper(n, k, target, &mut dp) as i32
	}

	fn num_rolls_to_target_helper(
		n: usize,
		k: usize,
		target: usize,
		dp: &mut Vec<Vec<usize>>,
	) -> usize {
		// n == 0 && target == 0 -> 1
		// f(0) = 0
		// f(2, 7) = f(1,6) + f(1,5) + f(1,4) + f(1,3) + f(1,2) + f(1,1)
		// f(n, target) = f(n-1, target-i)

		if n == 0 && target == 0 {
			1
		} else if n == 0 {
			0
		} else if dp[n][target] != usize::MAX {
			dp[n][target]
		} else {
			let mut take = 0;

			for i in 1..=k {
				if target >= i {
					take += Self::num_rolls_to_target_helper(n - 1, k, target - i, dp);
					take %= 1000000007;
				}
			}

			dp[n][target] = take;
			take
		}
	}
}
