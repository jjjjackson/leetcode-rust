use crate::solutions::Solution;

impl Solution {
	pub fn reverse_words(s: String) -> String {
		let reverse_word = |slice: &str| {
			slice
				.chars()
				.collect::<Vec<char>>()
				.into_iter()
				.rev()
				.collect::<String>()
		};

		s.split(" ")
			.into_iter()
			.map(reverse_word)
			.collect::<Vec<String>>()
			.join(" ")
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case("Let's take LeetCode contest", "s'teL ekat edoCteeL tsetnoc")]
	fn cases(s: &str, expected: &str) {
		assert_eq!(
			super::Solution::reverse_words(s.to_string()),
			expected.to_string()
		);
	}
}
