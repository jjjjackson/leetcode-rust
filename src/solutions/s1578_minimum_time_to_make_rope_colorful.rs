use crate::solutions::Solution;

impl Solution {
	pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
		if colors.is_empty() {
			return 0;
		}

		let mut slices: Vec<(usize, usize)> = Vec::new();

		let bytes = colors.as_bytes();
		let mut first_char_position = 0;
		for i in 1..(bytes.len()) {
			if bytes[first_char_position] != bytes[i] {
				slices.push((first_char_position, i - 1));
				first_char_position = i;
			} else if i == bytes.len() - 1 && bytes[first_char_position] == bytes[i] {
				slices.push((first_char_position, i));
			}
		}

		let mut total_time = 0;
		for slice in slices {
			if slice.1 - slice.0 != 0 {
				let mut time = 0;
				let mut max = i32::MIN;
				for i in (slice.0)..=(slice.1) {
					time += needed_time[i];
					max = max.max(needed_time[i]);
				}
				total_time += time - max;
			}
		}

		total_time
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case("abaac", vec![1,2,3,4,5], 3)]
	#[test_case("abc", vec![1,2,3], 0)]
	#[test_case("aabaa", vec![1,2,3,4,1], 2)]
	fn cases(colors: &str, needed_time: Vec<i32>, expected: i32) {
		assert_eq!(
			super::Solution::min_cost(colors.to_string(), needed_time),
			expected
		);
	}
}
