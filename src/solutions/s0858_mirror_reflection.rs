// https://leetcode.com/problems/mirror-reflection/
use crate::solutions::Solution;

// Time: O(log p)
// Space: O(1)
// 858
impl Solution {
	pub fn mirror_reflection(p: i32, q: i32) -> i32 {
		let mut m = 1;
		let mut n = 1;
		while m * p != n * q {
			n += 1;
			m = n * q / p;
		}

		if m % 2 == 0 && n % 2 == 1 {
			0
		} else if m % 2 == 1 && n % 2 == 1 {
			1
		} else if m % 2 == 1 && n % 2 == 0 {
			2
		} else {
			-1
		}
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(2, 1, 2)]
	#[test_case(3, 1, 1)]
	#[test_case(4, 3, 2)]
	#[test_case(3, 2, 0)]
	pub fn success_test(p: i32, q: i32, expect: i32) {
		assert_eq!(super::Solution::mirror_reflection(p, q), expect);
	}
}
