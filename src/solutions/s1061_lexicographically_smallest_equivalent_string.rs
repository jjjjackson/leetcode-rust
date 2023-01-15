struct Solution {}

impl Solution {
	fn find(char_union: &mut Vec<usize>, a: usize) -> usize {
		if char_union[a] == usize::MAX {
			char_union[a] = a;
			a
		} else {
			let mut b = a;
			while char_union[b] != b {
				b = char_union[b];
			}
			char_union[a] = b;
			b
		}
	}

	fn union(char_union: &mut Vec<usize>, a: usize, b: usize) {
		let a = Self::find(char_union, a);
		let b = Self::find(char_union, b);
		if a < b {
			char_union[b] = a;
		} else {
			char_union[a] = b;
		}
	}

	pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
		let mut char_union: Vec<usize> = vec![usize::MAX; 26];
		let s1_chars = s1
			.bytes()
			.map(|i| (i - b'a') as usize)
			.collect::<Vec<usize>>();
		let s2_chars = s2
			.bytes()
			.map(|i| (i - b'a') as usize)
			.collect::<Vec<usize>>();

		for i in 0..s1_chars.len() {
			Self::union(&mut char_union, s1_chars[i], s2_chars[i]);
		}

		base_str
			.bytes()
			.map(|c| ((Self::find(&mut char_union, (c - b'a') as usize) as u8) + b'a') as char)
			.collect::<_>()
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case("parker", "morris", "parser", "makkek")]
	fn cases(s1: &str, s2: &str, base_str: &str, expected: &str) {
		assert_eq!(
			super::Solution::smallest_equivalent_string(
				s1.to_string(),
				s2.to_string(),
				base_str.to_string()
			),
			expected.to_string()
		);
	}
}
