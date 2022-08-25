use crate::solutions::Solution;

impl Solution {
	pub fn can_construct(ransom_note: String, magazine: String) -> bool {
		let mut r_count: Vec<i32> = vec![0; 26];
		let mut m_count: Vec<i32> = vec![0; 26];
		let a_byte = "a".as_bytes()[0];

		for b in ransom_note.as_bytes() {
			r_count[(b - a_byte) as usize] += 1;
		}

		for b in magazine.as_bytes() {
			m_count[(b - a_byte) as usize] += 1;
		}

		for i in 0..26 {
			if r_count[i] > m_count[i] {
				return false;
			}
		}

		true
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case("aa", "aa", true)]
	#[test_case("aa", "ab", false)]
	#[test_case("aa", "aab", true)]
	fn success_case(ransom_note: &str, magazine: &str, expected: bool) {
		assert_eq!(
			super::Solution::can_construct(
				ransom_note.to_string(),
				magazine.to_string()
			),
			expected
		);
	}
}
