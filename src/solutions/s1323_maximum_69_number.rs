use crate::solutions::Solution;

impl Solution {
	pub fn maximum69_number(num: i32) -> i32 {
		let mut string: Vec<char> = num.to_string().chars().collect();

		let index = string.iter().position(|&x| x == '6').unwrap_or_default();
		string[index] = '9';

		string.iter().collect::<String>().parse::<i32>().unwrap()
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(9669, 9969)]
	fn cases(num: i32, expected: i32) {
		assert_eq!(super::Solution::maximum69_number(num), expected);
	}
}
