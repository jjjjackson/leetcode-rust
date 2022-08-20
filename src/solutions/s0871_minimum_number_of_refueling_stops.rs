// https://leetcode.com/problems/minimum-number-of-refueling-stops/submissions/
// 有趣的貪心題

use crate::solutions::Solution;

use std::collections::BinaryHeap;
impl Solution {
	pub fn min_refuel_stops(
		target: i32,
		start_fuel: i32,
		stations: Vec<Vec<i32>>,
	) -> i32 {
		if start_fuel >= target {
			return 0;
		}

		let mut queue: BinaryHeap<i32> = BinaryHeap::new();
		let mut max_dist = start_fuel;
		let mut stops = 0;
		let mut i = 0;

		while max_dist < target {
			while i < stations.len() && stations[i][0] <= max_dist {
				queue.push(stations[i][1]);
				i += 1;
			}
			if queue.is_empty() {
				return -1;
			}

			max_dist += queue.pop().unwrap_or_default();
			stops += 1;
		}

		stops
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(1, 1, vec![], 0)]
	#[test_case(100, 1, vec![], -1)]
	#[test_case(100, 1, vec![vec![10,100]], -1)]
	#[test_case(100, 50, vec![vec![25,30]], -1)]
	#[test_case(100, 10, vec![vec![10,60], vec![20,30], vec![30,30],vec![60,40]], 2)]
	#[test_case(1000, 299, vec![vec![13,21],vec![26,115],vec![100,47],vec![225,99],vec![299,141],vec![444,198],vec![608,190],vec![636,157],vec![647,255],vec![841,123]], 4)]
	fn success(
		target: i32,
		start_fuel: i32,
		stations: Vec<Vec<i32>>,
		expected: i32,
	) {
		assert_eq!(
			super::Solution::min_refuel_stops(target, start_fuel, stations),
			expected
		);
	}
}
