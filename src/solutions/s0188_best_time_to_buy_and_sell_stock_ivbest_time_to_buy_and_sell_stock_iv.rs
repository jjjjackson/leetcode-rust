use crate::solutions::Solution;

impl Solution {
	pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
		if k == 0 {
			return 0;
		}
		if prices.is_empty() || prices.len() == 1 {
			return 0;
		}

		let mut dp = vec![vec![0; prices.len()]; (k + 1) as usize];
		for i in 0..dp.len() {
			for j in 0..dp[i].len() {
				if i == 0 || j == 0 {
					dp[i][j] = 0;
				} else {
					let mut max = i32::MIN;
					for k in 0..j {
						let x = prices[j] - prices[k] + dp[i - 1][k];
						max = x.max(max);
					}

					dp[i][j] = max.max(dp[i][j - 1]);
				}
			}
		}

		dp[dp.len() - 1][prices.len() - 1]
	}
}
