use crate::solutions::Solution;
impl Solution {
	pub fn sum_even_after_queries(
		nums: Vec<i32>,
		queries: Vec<Vec<i32>>,
	) -> Vec<i32> {
		let mut nums = nums;
		let mut results: Vec<i32> = Vec::new();

		for query in queries {
			nums[query[1] as usize] += query[0];

			let mut result = 0;
			for n in nums.clone() {
				if n % 2 == 0 {
					result += n;
				}
			}

			results.push(result);
		}

		results
	}
}
