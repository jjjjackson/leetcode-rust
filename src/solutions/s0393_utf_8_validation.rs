// https://leetcode.com/problems/utf-8-validation

use crate::solutions::Solution;

impl Solution {
	fn calc(mut val: i32) -> i32 {
		let (mask, mut n) = (128, 0);
		while (mask & val) != 0 && n != 4 {
			val = val << 1;
			n += 1;
		}

		if n == 1 {
			-1
		} else {
			n
		}
	}

	pub fn valid_utf8(data: Vec<i32>) -> bool {
		let (mask, mut n) = (128, 0);

		for i in 0..data.len() {
			if n != 0 {
				if Self::calc(data[i]) != -1 {
					return false;
				} else {
					n -= 1;
				}
			} else {
				n = if (mask & data[i]) == 0 {
					0
				} else {
					Self::calc(data[i]) - 1
				};
				if n < 0 || i as i32 + n >= data.len() as i32 {
					return false;
				}
				if n > 0 && ((mask >> n + 1) & data[i] != 0) {
					return false;
				}
			}
		}

		return n == 0;
	}
}
