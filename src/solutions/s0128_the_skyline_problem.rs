use crate::solutions::Solution;

// Time Limit Exceeded
impl Solution {
	pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		use std::collections::HashMap;

		let mut max_height_of_nodes: HashMap<i32, i32> = HashMap::new();
		let mut max_position = 0;

		for v in buildings {
			for i in v[0]..v[1] {
				let value = max_height_of_nodes.get(&i).unwrap_or(&0i32);
				max_height_of_nodes.insert(i, *value.max(&v[2]));
				max_position = max_position.max(v[1]);
			}
		}

		let mut prev = 0;
		let mut result = vec![];
		for i in 0..=max_position {
			let position = max_height_of_nodes.get(&i).unwrap_or(&0);
			if position != &prev {
				result.push(vec![i, *position]);
				prev = *position;
			}
		}
		result
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(vec![vec![2,9,10],vec![3,7,15],vec![5,12,12],vec![15,20,10],vec![19,24,8]], vec![vec![2,10],vec![3,15],vec![7,12],vec![12,0],vec![15,10],vec![20,8],vec![24,0]])]
	#[test_case(vec![vec![0,2,3],vec![2,5,3]], vec![vec![0,3],vec![5,0]])]
	fn cases(buildings: Vec<Vec<i32>>, expected: Vec<Vec<i32>>) {
		assert_eq!(super::Solution::get_skyline(buildings), expected);
	}
}
