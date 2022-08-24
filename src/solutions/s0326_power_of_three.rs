// https://leetcode.com/problems/power-of-three/
use crate::solutions::Solution;

impl Solution {
	pub fn is_power_of_three(n: i32) -> bool {
		let mut temp = n as f64;
		while temp > 1f64 {
			temp /= 3f64;
		}

		if temp == 1f64 {
			true
		} else {
			false
		}
	}
}
#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(9, true)]
	#[test_case(5, false)]
	#[test_case(1, true)]
	#[test_case(0, false)]
	fn success_cases(n: i32, expected: bool) {
		assert_eq!(super::Solution::is_power_of_three(n), expected);
	}
}
