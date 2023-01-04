struct Solution {}

use std::collections::HashMap;

impl Solution {
	pub fn minimum_rounds_counter(mut frequency: i32) -> i32 {
		if frequency == 1 {
			return -1;
		}

		let three = frequency / 3;
		frequency -= three * 3;
		let mut two = frequency / 2;
		frequency -= two * 2;
		if frequency == 1 {
			two += 1;
		}

		three + two
	}

	pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
		let mut map: HashMap<i32, i32> = HashMap::new();
		tasks.iter().for_each(|value| {
			*map.entry(*value).or_default() += 1;
		});

		let counters = map
			.values()
			.into_iter()
			.map(|freq| Self::minimum_rounds_counter(*freq))
			.collect::<Vec<_>>();
		if counters.iter().any(|c| *c == -1) {
			-1
		} else {
			counters.iter().sum()
		}
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(vec![2,2,3,3,2,4,4,4,4,4], 4)]
	#[test_case(vec![2,3,3], -1)]
	fn cases(tasks: Vec<i32>, expected: i32) {
		assert_eq!(super::Solution::minimum_rounds(tasks), expected);
	}
}
