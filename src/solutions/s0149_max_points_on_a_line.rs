struct Solution {}

impl Solution {
	pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
		let get_slot = |x: Vec<i32>, y: Vec<i32>| -> f32 {
			if x[0] - y[0] == 0 {
				f32::MAX
			} else {
				((x[1] - y[1]) as f32) / ((x[0] - y[0]) as f32)
			}
		};

		let mut max_number_of_points_on_line = 1;
		for x_index in 0..points.len() {
			let x = points[x_index].clone();
			for y_index in 0..points.len() {
				if x_index != y_index {
					let y = points[y_index].clone();
					let slot_x_y = get_slot(x.to_vec(), y.to_vec());
					let mut number_of_points_on_line = 2;
					for z_index in 0..points.len() {
						let z = points[z_index].clone();
						if z_index != y_index
							&& z_index != x_index && get_slot(x.to_vec(), z.to_vec()) == slot_x_y
						{
							number_of_points_on_line += 1;
						}
					}
					max_number_of_points_on_line =
						max_number_of_points_on_line.max(number_of_points_on_line);
				}
			}
		}

		max_number_of_points_on_line
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(vec![[1,1],[2,2],[3,3]], 3)]
	#[test_case(vec![[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]], 4)]
	#[test_case(vec![[0,1],[0,0]], 2)]
	#[test_case(vec![[0,0]], 1)]
	fn cases(points: Vec<[i32; 2]>, expected: i32) {
		assert_eq!(
			super::Solution::max_points(points.into_iter().map(|x| x.to_vec()).collect::<Vec<_>>()),
			expected
		);
	}
}
