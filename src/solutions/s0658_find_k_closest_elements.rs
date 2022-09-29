use crate::solutions::Solution;

// binary search

impl Solution {
	pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
		let k = k as usize;
		let (mut low, mut high) = (0, arr.len() - k);

		while low < high {
			let middle = low + (high - low) / 2;
			if x - arr[middle] > arr[middle + k] - x {
				low = middle + 1;
			} else {
				high = middle;
			}
		}
		Vec::from(&arr[low..low + k])
	}
}

// 7%
// impl Solution {
// 	pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
// 		use std::collections::BinaryHeap;

// 		let mut map = arr
// 			.iter()
// 			.map(|node| ((x - node).abs(), *node))
// 			.collect::<BinaryHeap<(i32, i32)>>();
// 		let mut result = vec![];
// 		(0..(map.len() - (k as usize))).for_each(|_| {
// 			let _ = map.pop();
// 		});
// 		(0..map.len()).for_each(|_| {
// 			result.push(map.pop().unwrap().1);
// 		});
// 		result.sort();
// 		result
// 	}
// }

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(vec![1,2,3,4,5], 4, 3, vec![1,2,3,4])]
	#[test_case(vec![1,2,3,4,5], 4, -1, vec![1,2,3,4])]
	#[test_case(vec![0,2,3,4,5], 4, 3, vec![2,3,4,5])]
	fn cases(arr: Vec<i32>, k: i32, x: i32, expected: Vec<i32>) {
		assert_eq!(super::Solution::find_closest_elements(arr, k, x), expected);
	}
}
