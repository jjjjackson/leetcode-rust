use crate::solutions::Solution;
impl Solution {
	pub fn concatenated_binary(n: i32) -> i32 {
		const DIV: u64 = 1_000_000_007;
		(2..=n).fold(1_u64, |acc, x| {
			((acc << (32 - x.leading_zeros())) + x as u64) % DIV
		}) as i32
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(3, 27)]
	#[test_case(12, 505379714)]
	#[test_case(13, 86075381)]
	#[test_case(42, 727837408)]
	fn cases(n: i32, expected: i32) {
		assert_eq!(super::Solution::concatenated_binary(n), expected);
	}
}
