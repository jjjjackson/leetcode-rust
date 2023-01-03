struct Solution {}

impl Solution {
	pub fn is_sorted(strs: Vec<char>) -> bool {
		strs.windows(2).all(|w| w[0] <= w[1])
	}

	pub fn min_deletion_size(strs: Vec<String>) -> i32 {
		let mut result = 0;

		if strs.is_empty() {
			return result;
		}

		for col_nums in 0..strs[0].len() {
			let row_str = strs
				.clone()
				.into_iter()
				.map(|s| {
					s.get(col_nums..col_nums + 1)
						.unwrap_or_default()
						.to_string()
				})
				.collect::<String>();
			let row_chars = row_str.chars().collect::<Vec<char>>();
			if !Self::is_sorted(row_chars) {
				result += 1;
			}
		}

		result
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(vec!["cba", "daf", "ghi"], 1)]
	#[test_case(vec!["a", "b"], 0)]
	#[test_case(vec!["zyx", "wvu", "tsr"], 3)]
	fn cases(strs: Vec<&str>, expected: i32) {
		assert_eq!(
			super::Solution::min_deletion_size(
				strs.iter().map(|s| s.to_string()).collect::<Vec<_>>()
			),
			expected
		);
	}
}
