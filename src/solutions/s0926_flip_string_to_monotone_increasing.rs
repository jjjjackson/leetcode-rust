struct Solution {}

impl Solution {
	// 其實沒看懂這個作法
	pub fn min_flips_mono_incr(s: String) -> i32 {
		s.chars()
			.fold((0, 0), |(ones, x), c| match c {
				'0' => (ones, ones.min(x + 1)),
				_ => (ones + 1, x),
			})
			.1
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case("00110", 1)]
	fn cases(s: &str, expected: i32) {
		assert_eq!(
			super::Solution::min_flips_mono_incr(s.to_string()),
			expected
		);
	}
}
