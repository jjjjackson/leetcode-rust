use crate::solutions::Solution;

use std::collections::HashMap;

impl Solution {
	pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
		if words.len() == 0 {
			return vec![];
		}

		let mut result = vec![];
		let mut original_words_map: HashMap<&String, i32> = HashMap::new();
		words.iter().for_each(|s| {
			match original_words_map.get(s) {
				Some(count) => original_words_map.insert(s, count + 1),
				None => original_words_map.insert(s, 1),
			};
		});

		let word_len = words[0].len();
		let total_word_len = word_len * words.len();

		for i in 0..(s.len() - word_len + 1) {
			let mut words_map = words
				.iter()
				.map(|s| (s, 0))
				.collect::<HashMap<&String, i32>>();

			let mut j = i;
			while j < (s.len() - word_len + 1) {
				match words.iter().find(|w| {
					j + word_len <= s.len()
						&& s.as_bytes()[j..j + word_len] == *(w.as_bytes())
				}) {
					None => break,
					Some(word) => {
						let count =
							words_map.get(word).unwrap_or_else(|| &0) + 1;
						words_map.insert(word, count);

						if j + word_len >= i + total_word_len {
							if words_map == original_words_map {
								result.push(i as i32);
							}
							break;
						} else {
							j += word_len;
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

	#[test_case("barfoothefoobarman", vec!["foo", "bar"], vec![0, 9])]
	#[test_case("wordgoodgoodgoodbestword", vec!["word","good","best","word"], vec![])]
	#[test_case("barfoofoobarthefoobarman", vec!["bar","foo","the"], vec![6, 9, 12])]
	#[test_case("aaaaaaaaa", vec!["aaa"], vec![0,1,2,3,4,5,6])]

	fn test(s: &str, words: Vec<&str>, expected: Vec<i32>) {
		assert_eq!(
			super::Solution::find_substring(
				s.to_string(),
				words.iter().map(|s| s.to_string()).collect::<_>()
			),
			expected
		);
	}
}
