use crate::solution::Solution;

// 458. Poor Pigs
// 如果沒有做過的話, 需要直接去看思路
impl Solution {
	pub fn poor_pigs(
		buckets: i32,
		minutes_to_die: i32,
		minutes_to_test: i32,
	) -> i32 {
		let frame = (minutes_to_test / minutes_to_die) as i32 + 1;

		let mut i = 0;
		while frame.pow(i) < buckets {
			i += 1;
		}

		i as i32
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test_case::test_case;

	#[test_case(1000, 15, 60, 5)]
	#[test_case(4, 15, 15, 2)]
	#[test_case(4, 15, 30, 2)]
	#[test_case(1, 1, 1, 0)]
	#[test_case(2, 1, 1, 1)]
	fn test_poor_pigs(
		buckets: i32,
		minutes_to_die: i32,
		minutes_to_test: i32,
		expected: i32,
	) {
		assert_eq!(
			Solution::poor_pigs(buckets, minutes_to_die, minutes_to_test),
			expected
		);
	}
}
