use crate::solutions::Solution;

impl Solution {
	pub fn reverse_vowels(s: String) -> String {
		let mut vowels = vec![];
		let is_vowel = |c| -> bool {
			match c {
				'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
				_ => false,
			}
		};

		s.chars().for_each(|c| {
			if is_vowel(c) {
				vowels.push(c);
			}
		});

		let mut result = vec![];
		let mut j = vowels.len() - 1;

		s.chars().for_each(|c| {
			if is_vowel(c) {
				result.push(vowels[j]);
				j -= 1;
			} else {
				result.push(c);
			}
		});

		result.iter().collect::<String>()
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case("hello", "holle")]
	#[test_case("aA", "Aa")]
	fn cases(s: &str, expected: &str) {
		assert_eq!(
			super::Solution::reverse_vowels(s.to_string()),
			expected.to_string()
		);
	}
}
