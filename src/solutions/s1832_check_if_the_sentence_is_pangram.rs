use crate::solutions::Solution;

impl Solution {
	pub fn check_if_pangram(sentence: String) -> bool {
		let mut alphabets = vec![false; 26];
		let a = 'a' as u8;

		sentence.chars().for_each(|c| {
			alphabets[((c as u8) - a) as usize] = true;
		});

		alphabets.iter().all(|&alphabet| alphabet)
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case("thequickbrownfoxjumpsoverthelazydog", true)]
	#[test_case("leetcode", false)]
	fn cases(sentence: &str, expected: bool) {
		assert_eq!(
			super::Solution::check_if_pangram(sentence.to_string()),
			expected
		);
	}
}
