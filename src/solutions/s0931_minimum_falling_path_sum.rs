pub struct Solution {}

impl Solution {
	pub fn min_falling_path_sum(_matrix: Vec<Vec<i32>>) -> i32 {
		0
	}
}

// const DIRECTIONS: [[i32;2];3] = [[1, -1], [1,0], [1,1]];
// struct Cell {
//     x: usize,
//     y: usize,
//     x_len: usize,
//     y_len: usize,
// }

// impl Cell {
//     pub fn next_cells(&self) -> Vec<Cell> {
//         let is_in_matrix = |x, y| { x >= 0 && y >=0 &&  x < self.x_len as i32 && y < self.y_len as i32};
//         DIRECTIONS
//             .into_iter()
//             .filter(|d| is_in_matrix(self.x as i32 + d[0], self.y as i32 + d[1]))
//             .map(|d| Cell {
//                 x: (self.x as i32 + d[0]) as usize,
//                 y: (self.y as i32 + d[1]) as usize,
//                 x_len: self.x_len,
//                 y_len: self.y_len,
//             })
//             .collect::<Vec<Cell>>()
//     }
// }
// impl Solution {
//     pub fn dfs(matrix: &Vec<Vec<i32>>, cell: Cell) -> i32 {
//         cell.next_cells()
//             .into_iter()
//             .map(|d| {
//                 Self::dfs(matrix, d) + matrix[cell.x][cell.y]
//             })
//             .min()
//             .unwrap_or_default()
//     }

//     pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
//         let mut min_sum = i32::MAX;

//         for i in 0..matrix[0].len() {
//             let cell = Cell {
//                 x: 0,
//                 y: i,
//                 x_len: matrix.len(),
//                 y_len: matrix[0].len(),
//             };
//             min_sum = min_sum.min(Self::dfs(&matrix, cell));
//         }

//         min_sum
//     }
// }

#[cfg(test)]
mod tests {
	use test_case::test_case;

	fn cases() {}
}
