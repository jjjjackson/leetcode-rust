use crate::solutions::Solution;

impl Solution {
	pub fn num_decodings(s: String) -> i32 {
		if s.is_empty() {
			return 0;
		}

		let numbers = s.as_bytes().iter().map(|a| *a - 48).collect::<Vec<u8>>();
		let mut dp = vec![0; numbers.len()];

		for i in (0..numbers.len()).rev() {
			if numbers[i] == 0 {
				continue;
			}
			if i + 1 == numbers.len() {
				dp[i] = 1;
				continue;
			}
			let mut v = dp[i + 1];
			if numbers[i] * 10 + numbers[i + 1] <= 26 && numbers[i] * 10 + numbers[i + 1] >= 10 {
				if i + 2 < numbers.len() {
					v += dp[i + 2];
				} else {
					v += 1;
				}
			}
			dp[i] = v;
		}

		dp[0]
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case("10", 1)]
	#[test_case("12", 2)]
	#[test_case("226", 3)]
	#[test_case("06", 0)]
	fn cases(s: &str, expected: i32) {
		assert_eq!(super::Solution::num_decodings(s.to_string()), expected);
	}
}
