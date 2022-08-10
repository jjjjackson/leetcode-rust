// https://leetcode.com/problems/valid-anagram/
use crate::solutions::Solution;
use std::collections::HashMap;

impl Solution {
	pub fn convert_string_to_hashmap(s: String) -> HashMap<char, i32> {
		let mut s_map: HashMap<char, i32> = HashMap::new();

		for c in s.chars() {
			let c_count = match s_map.get(&c) {
				Some(num) => num + 1,
				None => 1,
			};
			s_map.insert(c, c_count);
		}
		s_map
	}
	pub fn is_anagram(s: String, t: String) -> bool {
		let s_map = Solution::convert_string_to_hashmap(s);
		let t_map = Solution::convert_string_to_hashmap(t);
		s_map == t_map
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test_case::test_case;

	#[test_case("abc", "bca")]
	#[test_case("a", "a")]
	#[test_case("aa", "aa")]
	#[test_case("aba", "aba")]
	#[test_case("ba", "ab")]
	fn success_cases(s: &str, t: &str) {
		assert!(Solution::is_anagram(s.to_string(), t.to_string()));
	}

	#[test_case("abc", "bcaaaaa")]
	fn fail_cases(s: &str, t: &str) {
		assert!(!Solution::is_anagram(s.to_string(), t.to_string()));
	}
}
