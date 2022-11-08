use crate::solutions::Solution;

impl Solution {
	pub fn make_good(s: String) -> String {
		let mut stack: Vec<char> = vec![];

		for c in s.chars() {
			match stack.last() {
				Some(last) if *last != c && last.to_ascii_lowercase() == c.to_ascii_lowercase() => {
					stack.pop();
				}
				_ => stack.push(c),
			}
		}

		stack.iter().collect::<String>()
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case("leEeetcode", "leetcode")]
	#[test_case("s", "s")]
	fn cases(s: &str, expected: &str) {
		assert_eq!(
			super::Solution::make_good(s.to_string()),
			expected.to_string()
		);
	}
}
