use crate::solutions::Solution;

impl Solution {
	pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
		let (m, n) = (matrix.len(), matrix[0].len());
		let mut dp = vec![vec![0; n]; m + 1];

		for i in 0..m {
			for j in 0..n {
				dp[i + 1][j] = dp[i][j] + matrix[i][j];
			}
		}

		let mut result = i32::MIN;

		for i in 0..m {
			for l in (i + 1)..=m {
				for j in 0..n {
					let mut val = 0;
					for k in j..n {
						val += dp[l][k] - dp[i][k];

						if val < target {
							result = std::cmp::max(result, val);
						} else if val == target {
							return target;
						}
					}
				}
			}
		}

		result
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(vec![vec![1,0,1],vec![0,-2,3]], 2, 2)]
	fn success(matrix: Vec<Vec<i32>>, target: i32, expected: i32) {
		assert_eq!(
			super::Solution::max_sum_submatrix(matrix, target),
			expected
		);
	}
}
