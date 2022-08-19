// https://leetcode.com/problems/reduce-array-size-to-the-half/

use crate::solutions::Solution;

use std::collections::{BinaryHeap, HashMap};
impl Solution {
	pub fn min_set_size(arr: Vec<i32>) -> i32 {
		let mut map: HashMap<i32, usize> = HashMap::new();
		for v in arr.clone() {
			*map.entry(v).or_default() += 1;
		}

		let mut heap: BinaryHeap<_> = map.values().collect();
		let mut result = 0;
		let mut counter = arr.len();
		while counter > (arr.len() / 2) {
			counter -= heap.pop().unwrap_or(&0);
			result += 1;
		}

		result
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(vec![3,3,3,3,5,5,5,2,2,7], 2)]
	#[test_case(vec![7,7,7,7,7,7,7,7], 1)]
	#[test_case(vec![1,9], 1)]
	#[test_case(vec![1000,1000,3,7], 1)]
	#[test_case(vec![1,2,3,4,5,6,7,8,9,10], 5)]
	fn success(arr: Vec<i32>, expected: i32) {
		assert_eq!(super::Solution::min_set_size(arr), expected);
	}
}
