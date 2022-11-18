use crate::solutions::Solution;

impl Solution {
	pub fn is_ugly(n: i32) -> bool {
		let mut n = n;
		let div = [2, 3, 5];
		while n > 1 {
			let d = div.iter().find(|d| n % *d == 0);
			match d {
				Some(d) => n = n / d,
				None => return false,
			}
		}

		n == 1
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(6, true)]
	#[test_case(1, true)]
	#[test_case(14, false)]
	fn cases(n: i32, expected: bool) {
		assert_eq!(super::Solution::is_ugly(n), expected);
	}
}
