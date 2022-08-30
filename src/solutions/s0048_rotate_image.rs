// https://leetcode.com/problems/rotate-image/
use crate::solutions::Solution;

impl Solution {
	pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
		for i in 1..matrix.len() {
			for j in 0..i {
				let t = matrix[i][j];
				matrix[i][j] = matrix[j][i];
				matrix[j][i] = t;
			}
		}
		matrix.iter_mut().for_each(|row| row.reverse());
	}
}
