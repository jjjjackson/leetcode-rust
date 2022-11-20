use crate::solutions::Solution;

impl Solution {
	pub fn calculate(s: String) -> i32 {
		let mut result = 0;

		let mut cur_num: i32 = 0;
		let mut sign: i32 = 1;

		let mut stack = vec![];
		stack.push(sign);

		for c in s.chars() {
			match c {
				'0'..='9' => {
					cur_num = cur_num * 10 + (c as i32 - '0' as i32);
				}
				'+' | '-' => {
					result += sign * cur_num;
					cur_num = 0;
					sign = stack.last().unwrap() * if c == '+' { 1 } else { -1 };
				}
				'(' => {
					stack.push(sign);
				}
				')' => {
					stack.pop();
				}
				_ => (),
			}
		}

		result + sign * cur_num
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case("1 + 1", 2)]
	#[test_case("(1+(4+5+2)-3)+(6+8)", 23)]
	fn cases(s: &str, expected: i32) {
		assert_eq!(super::Solution::calculate(s.to_string()), expected);
	}
}
