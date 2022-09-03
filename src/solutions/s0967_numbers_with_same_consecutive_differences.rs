// https://leetcode.com/problems/numbers-with-same-consecutive-differences/

use crate::solutions::Solution;

impl Solution {
	fn dfs(num: i32, digit: i32, n: i32, k: i32, numbers: &mut Vec<i32>) {
		if n == 0 {
			numbers.push(num);
		} else {
			if digit + k <= 9 {
				Self::dfs(num * 10 + digit + k, digit + k, n - 1, k, numbers);
			}
			if k != 0 && digit - k >= 0 {
				Self::dfs(num * 10 + digit - k, digit - k, n - 1, k, numbers);
			}
		}
	}

	pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
		let mut numbers: Vec<i32> = Vec::new();

		for num in 1..10 {
			Self::dfs(num, num, n - 1, k, &mut numbers);
		}

		numbers
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(3,7,vec![181,292,707,818,929])]
	#[test_case(2,1,vec![10,12,21,23,32,34,43,45,54,56,65,67,76,78,87,89,98])]
	fn success_case(n: i32, k: i32, expected: Vec<i32>) {
		let answer = super::Solution::nums_same_consec_diff(n, k);
		assert!(expected.iter().all(|f| answer.contains(f)));
	}
}
