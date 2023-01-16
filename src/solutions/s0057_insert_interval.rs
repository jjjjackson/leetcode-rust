struct Solution {}

impl Solution {
	pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
		let mut answer: Vec<Vec<i32>> = Vec::new();
		let mut index = 0;

		while index < intervals.len() && intervals[index][1] < new_interval[0] {
			answer.push(intervals[index].clone());
			index += 1;
		}

		let mut new_interval = new_interval;
		while index < intervals.len() && intervals[index][0] <= new_interval[1] {
			new_interval[0] = new_interval[0].min(intervals[index][0]);
			new_interval[1] = new_interval[1].max(intervals[index][1]);
			index += 1;
		}

		answer.push(new_interval.clone());

		while index < intervals.len() {
			answer.push(intervals[index].clone());
			index += 1;
		}

		answer
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(vec![[1,3],[6,9]], vec![2,5], vec![[1,5],[6,9]])]
	#[test_case(vec![[1,2],[3,5],[6,7],[8,10],[12,16]], vec![4,8], vec![[1,2],[3,10],[12,16]])]
	fn cases(intervals: Vec<[i32; 2]>, new_interval: Vec<i32>, expected: Vec<[i32; 2]>) {
		assert_eq!(
			super::Solution::insert(
				intervals.iter().map(|a| a.to_vec()).collect::<Vec<_>>(),
				new_interval
			),
			expected.iter().map(|a| a.to_vec()).collect::<Vec<_>>()
		);
	}
}
