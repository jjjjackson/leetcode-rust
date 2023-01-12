struct Solution {}

use std::collections::{BTreeMap, BTreeSet};

impl Solution {
	fn build_graph(edges: Vec<Vec<i32>>) -> BTreeMap<usize, BTreeSet<usize>> {
		let mut graph: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();

		edges.iter().for_each(|edge| {
			graph
				.entry(edge[0] as usize)
				.or_insert(BTreeSet::new())
				.insert(edge[1] as usize);
			graph
				.entry(edge[1] as usize)
				.or_insert(BTreeSet::new())
				.insert(edge[0] as usize);
		});

		graph
	}

	fn dfs(
		key: usize,
		parent: usize,
		graph: &BTreeMap<usize, BTreeSet<usize>>,
		labels: &Vec<char>,
		counters: BTreeMap<char, i32>,
		results: &mut Vec<i32>,
	) -> BTreeMap<char, i32> {
		let mut new_counters = counters.clone();
		graph.get(&key).unwrap().into_iter().for_each(|path| {
			if path != &parent {
				let children = Self::dfs(*path, key, graph, labels, counters.clone(), results);
				children.iter().for_each(|(key, value)| {
					*new_counters.entry(*key).or_default() += value;
				});
			}
		});

		*new_counters.entry(labels[key]).or_default() += 1;
		results[key] = *new_counters.get(&labels[key]).unwrap_or(&0);
		new_counters
	}

	pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
		let graph = Self::build_graph(edges);
		let labels = labels.chars().collect::<Vec<char>>();
		let mut result: Vec<i32> = vec![0; n as usize];

		Self::dfs(0, 0, &graph, &labels, BTreeMap::new(), &mut result);
		result
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(7, vec![[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], "abaedcd", vec![2,1,1,1,1,1,1])]
	#[test_case(4, vec![[0,1],[1,2],[0,3]], "bbbb", vec![4,2,1,1])]
	#[test_case(5, vec![[0,1],[0,2],[1,3],[0,4]], "aabab", vec![3,2,1,1,1])]
	#[test_case(7, vec![[0,1],[1,2],[2,3],[3,4],[4,5],[5,6]], "aaabaaa", vec![6,5,4,1,3,2,1])]
	fn cases(n: i32, edges: Vec<[i32; 2]>, labels: &str, expected: Vec<i32>) {
		assert_eq!(
			super::Solution::count_sub_trees(
				n,
				edges.iter().map(|e| e.to_vec()).collect::<Vec<_>>(),
				labels.to_string()
			),
			expected
		);
	}
}
