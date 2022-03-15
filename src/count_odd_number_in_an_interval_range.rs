use crate::solution::Solution;

impl Solution {
	pub fn count_odds(low: i32, high: i32) -> i32 {
		let real_low = if low < high { low } else { high };

		let real_high = if low < high { high } else { low };

		(real_low..real_high + 1)
			.into_iter()
			.filter(|i| i % 2 == 1)
			.count() as i32
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn normal() {
		assert_eq!(Solution::count_odds(1, 9), 5);
	}
}
