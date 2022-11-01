use std::convert::TryInto;

use crate::solutions::Solution;

impl Solution {
	pub fn find_ball_helper_move_ball(grid: &Vec<Vec<i32>>, j: usize) -> i32 {
		let (m, n) = (grid.len(), grid[0].len());
		let mut j = j;

		for i in 0..m {
			let k = (j as i32) + grid[i][j];
			if !(0 <= k && k < n as i32) || (grid[i][k as usize] != grid[i][j]) {
				return -1;
			}
			j = k.try_into().unwrap();
		}

		j as i32
	}

	pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
		if grid.is_empty() {
			return vec![];
		}

		let mut result = vec![];

		for j in 0..grid[0].len() {
			result.push(Self::find_ball_helper_move_ball(&grid, j));
		}

		result
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(vec![vec![1,1,1,-1,-1],vec![1,1,1,-1,-1],vec![-1,-1,-1,1,1],vec![1,1,1,1,-1],vec![-1,-1,-1,-1,-1]], vec![1,-1,-1,-1,-1]; "test_case_1")]
	#[test_case(vec![vec![-1]], vec![-1]; "test_case_2")]
	#[test_case(vec![vec![1,1,1,1,1,1],vec![-1,-1,-1,-1,-1,-1],vec![1,1,1,1,1,1],vec![-1,-1,-1,-1,-1,-1]],vec![0,1,2,3,4,-1]; "test_case_3")]
	fn cases(grid: Vec<Vec<i32>>, expected: Vec<i32>) {
		assert_eq!(super::Solution::find_ball(grid), expected);
	}
}
