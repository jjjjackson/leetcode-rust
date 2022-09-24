// https://leetcode.com/problems/pacific-atlantic-water-flow/
use crate::solutions::Solution;

use std::collections::{HashSet, VecDeque};
impl Solution {
	pub fn neighbors(row: i32, col: i32) -> Vec<(i32, i32)> {
		vec![
			(row + 1, col),
			(row - 1, col),
			(row, col + 1),
			(row, col - 1),
		]
	}

	pub fn can_flew_p_a(m: &Vec<Vec<i32>>, row: i32, col: i32) -> bool {
		let (mut if_reach_p, mut if_reach_a) = (false, false);
		let mut visited: HashSet<(i32, i32)> = HashSet::new();
		let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
		queue.push_front((row, col));

		while !queue.is_empty() {
			let (cur_r, cur_c) = queue.pop_front().unwrap();

			if !visited.contains(&(cur_r, cur_c)) {
				visited.insert((cur_r, cur_c));

				for (n_r, n_c) in Self::neighbors(cur_r, cur_c) {
					if n_r < 0 || n_c < 0 {
						if_reach_p = true;
					} else if n_r >= m.len() as i32 || n_c >= m[0].len() as i32 {
						if_reach_a = true;
					} else if !visited.contains(&(n_r, n_c))
						&& m[n_r as usize][n_c as usize] <= m[cur_r as usize][cur_c as usize]
					{
						queue.push_back((n_r, n_c));
					}
				}
			}

			if if_reach_p && if_reach_a {
				return true;
			}
		}

		false
	}

	pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		if heights.is_empty() || heights[0].is_empty() {
			return vec![vec![]];
		}

		let mut result: Vec<Vec<i32>> = vec![];
		for i in 0..heights.len() as i32 {
			for j in 0..heights[0].len() as i32 {
				if Self::can_flew_p_a(&heights, i, j) {
					result.push(vec![i, j]);
				}
			}
		}

		result
	}
}
