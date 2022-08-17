// https://leetcode.com/problems/unique-morse-code-words/
use crate::solutions::Solution;

use std::collections::HashMap;
impl Solution {
	pub fn unique_morse_representations(words: Vec<String>) -> i32 {
		let morse_codes: Vec<&str> = vec![
			".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..",
			".---", "-.-", ".-..", "--", "-.", "---", ".--.", "--.-", ".-.",
			"...", "-", "..-", "...-", ".--", "-..-", "-.--", "--..",
		];
		let a_byte = "a".to_string().as_bytes()[0];
		let mut exist_morse: HashMap<String, i32> = HashMap::new();

		for w in words {
			let morse_string = w
				.as_bytes()
				.iter()
				.map(|w| morse_codes[(w - a_byte) as usize])
				.collect::<String>();
			exist_morse.insert(
				morse_string.clone(),
				exist_morse.get(&morse_string).unwrap_or_else(|| &0) + 1,
			);
		}

		exist_morse.len() as i32
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(vec!["gin","zen","gig","msg"], 2)]
	#[test_case(vec!["a"], 1)]
	#[test_case(vec!["rwjje","aittjje","auyyn","lqtktn","lmjwn"], 1)]
	fn success(words: Vec<&str>, expected: i32) {
		assert_eq!(
			super::Solution::unique_morse_representations(
				words.iter().map(|s| s.to_string()).collect::<Vec<String>>()
			),
			expected
		);
	}
}
