struct Solution {}

use std::collections::{HashMap, HashSet};
impl Solution {
	pub fn dfs(
		edges: &HashMap<i32, HashSet<i32>>,
		current: i32,
		destination: i32,
		visited: &mut HashSet<i32>,
	) -> bool {
		if current == destination {
			true
		} else if let Some(set) = edges.get(&current) {
			set.into_iter()
				.any(|s| visited.insert(*s) && Self::dfs(edges, *s, destination, visited))
		} else {
			false
		}
	}

	pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
		let mut edges_map: HashMap<i32, HashSet<i32>> = HashMap::new();
		edges.into_iter().for_each(|edge| {
			edges_map
				.entry(edge[0])
				.or_insert(HashSet::new())
				.insert(edge[0]);
			edges_map
				.entry(edge[0])
				.or_insert(HashSet::new())
				.insert(edge[1]);
			edges_map
				.entry(edge[1])
				.or_insert(HashSet::new())
				.insert(edge[0]);
		});

		let mut visited = HashSet::new();
		visited.insert(source);
		Self::dfs(&edges_map, source, destination, &mut visited)
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(1, vec![], 0, 0, true)]
	#[test_case(3, vec![[0,1],[1,2],[2,0]], 0,2, true)]
	fn cases(n: i32, edges: Vec<[i32; 2]>, source: i32, destination: i32, expected: bool) {
		assert_eq!(
			super::Solution::valid_path(
				n,
				edges.into_iter().map(|a| a.to_vec()).collect::<Vec<_>>(),
				source,
				destination
			),
			expected
		);
	}
}
