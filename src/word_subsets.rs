use crate::solution::Solution;

impl Solution {
	pub fn convert_string_to_vector(s: String) -> Vec<u32> {
		let mut s_vec = vec![0u32; 26];

		s.chars().for_each(|c| {
			let digit = c as u32;
			s_vec[(digit - 'a' as u32) as usize] += 1;
		});

		s_vec
	}

	pub fn word_subsets(
		words1: Vec<String>,
		words2: Vec<String>,
	) -> Vec<String> {
		let word2_vec = words2
			.into_iter()
			.map(Solution::convert_string_to_vector)
			.reduce(|mut acc, vec| {
				vec.iter().enumerate().for_each(|(i, v)| {
					acc[i] = acc[i].max(*v);
				});
				acc
			})
			.unwrap();

		words1
			.into_iter()
			.filter(|word| {
				let word_vec = Solution::convert_string_to_vector(word.clone());
				word_vec.iter().enumerate().all(|(i, v)| v >= &word2_vec[i])
			})
			.collect::<_>()
	}
}

// impl Solution {
// 	pub fn string_to_hashmap(word: String) -> HashMap<char, i32> {
// 		let mut map: HashMap<char, i32> = HashMap::new();

// 		for c in word.chars() {
// 			let count = match map.get(&c) {
// 				Some(count) => count + 1,
// 				None => 1,
// 			};
// 			map.insert(c, count);
// 		}

// 		map
// 	}

// 	pub fn is_subset(
// 		word1: HashMap<char, i32>,
// 		word2: HashMap<char, i32>,
// 	) -> bool {
// 		for (key, val) in word2.iter() {
// 			match word1.get(key) {
// 				Some(count) if count < val => {
// 					return false;
// 				}
// 				None => {
// 					return false;
// 				}
// 				_ => {}
// 			}
// 		}
// 		return true;
// 	}

// 	pub fn has_all_subsets(
// 		word1: String,
// 		words: Vec<HashMap<char, i32>>,
// 	) -> bool {
// 		let word1_map = Solution::string_to_hashmap(word1);

// 		for w in words {
// 			if !Solution::is_subset(word1_map.clone(), w.clone()) {
// 				return false;
// 			}
// 		}
// 		return true;
// 	}

// 	pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
// 		let word2_map: Vec<HashMap<char, i32>> = words2
// 			.into_iter()
// 			.map(Solution::string_to_hashmap)
// 			.collect::<_>();

// 		words1
// 			.into_iter()
// 			.filter_map(|word| {
// 				if Solution::has_all_subsets(word.clone(), word2_map.clone()) {
// 					Some(word)
// 				} else {
// 					None
// 				}
// 			})
// 			.collect::<_>()
// 	}
// }

#[cfg(test)]
mod tests {
	use super::*;
	use test_case::test_case;

	#[test_case(vec!["amazon","apple","facebook","google","leetcode"], vec!["e","o"], vec!["facebook","google","leetcode"]
)]
	#[test_case(vec!["amazon","apple","facebook","google","leetcode"], vec!["l","e"], vec!["apple","google","leetcode"]
)]
	#[test_case(vec!["apple","facebook"], vec!["plp","e"], vec!["apple"]
)]
	fn success_cases(s: Vec<&str>, t: Vec<&str>, expected: Vec<&str>) {
		let words1: Vec<String> =
			s.iter().map(|s| s.to_string()).collect::<_>();
		let words2: Vec<String> =
			t.iter().map(|s| s.to_string()).collect::<_>();
		let expected: Vec<String> =
			expected.iter().map(|s| s.to_string()).collect::<_>();

		assert_eq!(
			Solution::word_subsets(words1.clone(), words2.clone()),
			expected
		);
	}
}
