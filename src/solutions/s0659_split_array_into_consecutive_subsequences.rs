// https://leetcode.com/problems/split-array-into-consecutive-subsequences/
// 有趣的貪心算法

use crate::solutions::Solution;

use std::collections::HashMap;
impl Solution {
	pub fn is_possible(nums: Vec<i32>) -> bool {
		let mut counter: HashMap<i32, i32> = HashMap::new();
		for i in nums.clone() {
			*counter.entry(i).or_default() += 1;
		}

		let mut seq: HashMap<i32, i32> = HashMap::new();
		for i in nums {
			if *counter.entry(i).or_default() == 0 {
				continue;
			} else if *seq.entry(i - 1).or_default() > 0 {
				*seq.entry(i - 1).or_default() -= 1;
				*seq.entry(i).or_default() += 1;
				*counter.entry(i).or_default() -= 1;
			} else if *counter.entry(i + 1).or_default() == 0
				|| *counter.entry(i + 2).or_default() == 0
			{
				return false;
			} else {
				*counter.entry(i).or_default() -= 1;
				*counter.entry(i + 1).or_default() -= 1;
				*counter.entry(i + 2).or_default() -= 1;
				*seq.entry(i + 2).or_default() += 1;
			}
		}

		true
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(vec![1,2,3,3,4,5,5,6,7], true)]
	#[test_case(vec![1,2,3,3,4,5], true)]
	#[test_case(vec![1,2,3,4,4,5], false)]
	fn success(nums: Vec<i32>, expected: bool) {
		assert_eq!(super::Solution::is_possible(nums), expected);
	}
}
