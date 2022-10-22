use crate::solutions::Solution;

use std::collections::BTreeMap;
impl Solution {
	pub fn is_include_all(s: &str, t: &str) -> bool {
		let mut s_map: BTreeMap<char, u16> = BTreeMap::new();
		s.chars().for_each(|c| {
			*s_map.entry(c).or_default() += 1;
		});
		let mut t_map: BTreeMap<char, u16> = BTreeMap::new();
		t.chars().for_each(|c| {
			*t_map.entry(c).or_default() += 1;
		});

		t_map
			.iter()
			.enumerate()
			.all(|(_, (key, value))| s_map.get(key).unwrap_or(&0u16) >= value)
	}
	pub fn min_window(s: String, t: String) -> String {
		if s.len() < t.len() {
			return "".to_string();
		}

		let (mut left, mut right) = (0, 0);
		let (mut min_left, mut min_right) = (0, 0);
		let (s, t) = (s.as_str(), t.as_str());

		while right <= s.len() {
			while Self::is_include_all(&s[left..right], t) {
				if (min_left == 0 && min_right == 0)
					|| ((min_left as i32 - min_right as i32).abs()
						> (left as i32 - right as i32).abs())
				{
					min_left = left;
					min_right = right;
				}
				left += 1;
			}
			right += 1;
		}

		s[min_left..min_right].to_string()
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case("ADOBECODEBANC", "ABC", "BANC")]
	#[test_case("aa", "aa", "aa")]
	#[test_case("a", "aa", "")]
	#[test_case("a", "b", "")]
	#[test_case("cabwefgewcwaefgcf", "cae", "cwae")]
	fn cases(s: &str, t: &str, expected: &str) {
		assert_eq!(
			super::Solution::min_window(s.to_string(), t.to_string()),
			expected
		);
	}
}
