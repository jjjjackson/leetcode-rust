use crate::solution::Solution;

impl Solution {
	fn convert(number: char) -> Option<i32> {
		match number {
			'I' => Some(1),
			'V' => Some(5),
			'X' => Some(10),
			'L' => Some(50),
			'C' => Some(100),
			'D' => Some(500),
			'M' => Some(1000),
			_ => None,
		}
	}

	pub fn roman_to_int(s: String) -> i32 {
		let mut result: i32 = 0;
		let mut previous_value = 0;
		for i in s.chars().map(Solution::convert) {
			let v = i.unwrap_or(0);
			if v > previous_value {
				result -= previous_value;
			} else {
				result += previous_value;
			}

			previous_value = v;
		}
		result + previous_value
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test_case::test_case;

	#[test_case("III", 3)]
	#[test_case("MCMXCIV", 1994)]
	fn normal(input: &str, expect: i32) {
		assert_eq!(Solution::roman_to_int(String::from(input)), expect);
	}
}
