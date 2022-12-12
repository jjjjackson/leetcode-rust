use crate::solutions::Solution;

impl Solution {
	pub fn climb_stairs(n: i32) -> i32 {
		// f(n) = f(n-1) + f(n-2)
		// f(0) = 1
		// f(1) = 1

		let (mut n_1, mut n_2) = (1, 1);
		let mut result = 1;

		for _ in 2..=n {
			result = n_1 + n_2;
			n_2 = n_1;
			n_1 = result;
		}

		result
	}
}
#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(0, 1)]
    #[test_case(2, 2)]
    #[test_case(3, 3)]
    #[test_case(4, 5)] 
	fn cases(n: i32, expected: i32) {
		assert_eq!(super::Solution::climb_stairs(n), expected);
	}
}
