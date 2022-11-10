use crate::solutions::Solution;

impl Solution {
	pub fn remove_duplicates(s: String) -> String {
		let mut stack = vec![];

		for c in s.chars() {
			match stack.last() {
				Some(last) if last == &c => {
					stack.pop();
				}
				_ => {
					stack.push(c);
				}
			}
		}

		stack.iter().collect::<String>()
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case("abbaca", "ca")]
	fn cases(s: &str, expected: &str) {
		assert_eq!(
			super::Solution::remove_duplicates(s.to_string()),
			expected.to_string()
		);
	}
}
