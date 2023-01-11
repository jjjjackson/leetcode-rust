struct Solution {}

// 沒有太懂這題
use std::collections::{BTreeSet, HashMap};

impl Solution {
	fn build_graph(n: i32, edges: &Vec<Vec<i32>>) -> HashMap<i32, BTreeSet<i32>> {
		let mut graph: HashMap<i32, BTreeSet<i32>> = HashMap::new();
		edges.into_iter().for_each(|v| {
			graph.entry(v[0]).or_default().insert(v[1]);
			graph.entry(v[1]).or_default().insert(v[0]);
		});
		graph
	}

	fn dfs(
		key: i32,
		graph: &HashMap<i32, BTreeSet<i32>>,
		has_apple: &Vec<bool>,
		parent: i32,
	) -> i32 {
		let mut total = 0;

		let paths = graph.get(&key).unwrap();
		paths
			.iter()
			.filter(|path| path != &&parent)
			.for_each(|path| {
				total += Self::dfs(*path, graph, has_apple, key);
			});

		if key != 0 && (has_apple[key as usize] || total > 0) {
			total + 2
		} else {
			total
		}
	}

	pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
		let graph = Self::build_graph(n, &edges);
		Self::dfs(0, &graph, &has_apple, 0)
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(7, vec![[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], vec![false,false,true,false,true,true,false], 8)]
	fn cases(n: i32, edges: Vec<[i32; 2]>, has_apple: Vec<bool>, expected: i32) {
		assert_eq!(
			super::Solution::min_time(
				n,
				edges.into_iter().map(|v| v.to_vec()).collect::<Vec<_>>(),
				has_apple
			),
			expected
		);
	}
}
