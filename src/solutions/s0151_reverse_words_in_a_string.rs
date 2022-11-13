use crate::solutions::Solution;

impl Solution {
	pub fn reverse_words_0151(s: String) -> String {
		s.split_whitespace()
			.collect::<Vec<_>>()
			.into_iter()
			.rev()
			.collect::<Vec<_>>()
			.join(" ")
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case("the sky is blue", "blue is sky the")]
	#[test_case("  hello world  ", "world hello")]
	#[test_case("a good   example", "example good a")]
	fn cases(s: &str, expected: &str) {
		assert_eq!(
			super::Solution::reverse_words_0151(s.to_string()),
			expected.to_string()
		);
	}
}
