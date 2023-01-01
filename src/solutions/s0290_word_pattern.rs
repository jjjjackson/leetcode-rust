struct Solution {}

use std::collections::HashMap;
impl Solution {
	pub fn word_pattern(pattern: String, s: String) -> bool {
		let mut s_pattern_counter = 0;
		let mut map: HashMap<String, u8> = HashMap::new();
		let s = s
			.split_whitespace()
			.into_iter()
			.map(|word| {
				let value = map.entry(word.to_string()).or_insert_with(|| {
					s_pattern_counter += 1;
					s_pattern_counter
				});
				value.to_string()
			})
			.collect::<String>();

		s_pattern_counter = 0;
		map.clear();
		let pattern = pattern
			.chars()
			.map(|word| {
				let value = map.entry(word.to_string()).or_insert_with(|| {
					s_pattern_counter += 1;
					s_pattern_counter
				});
				value.to_string()
			})
			.collect::<String>();

		s == pattern
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case("abba", "dog cat cat dog", true)]
	#[test_case("aaaa", "dog cat cat dog", false)]
	fn cases(pattern: &str, s: &str, expected: bool) {
		assert_eq!(
			super::Solution::word_pattern(pattern.to_string(), s.to_string()),
			expected
		);
	}
}
