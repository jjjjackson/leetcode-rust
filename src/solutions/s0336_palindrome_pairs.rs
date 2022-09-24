use crate::solutions::Solution;

// 自己寫的，會超時
impl Solution {
	fn is_palindrome_pairs(word: &str) -> bool {
		let word = word.chars().collect::<Vec<char>>();
		let (mut left, mut right) = (0, word.len() - 1);
		while left < right && word[left] == word[right] {
			left += 1;
			right -= 1;
		}
		left >= right
	}

	pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
		let mut result: Vec<Vec<i32>> = vec![];
		for a in 0..words.len() {
			for b in 0..words.len() {
				if a != b && Self::is_palindrome_pairs(format!("{}{}", words[a], words[b]).as_str())
				{
					result.push(vec![a as i32, b as i32]);
				}
			}
		}

		result
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(vec!["abcd","dcba","lls","s","sssll"], vec![vec![0,1],vec![1,0],vec![3,2],vec![2,4]])]
	#[test_case(vec!["bat","tab","cat"], vec![vec![0,1],vec![1,0]])]
	#[test_case(vec!["a", ""], vec![vec![0,1], vec![1,0]])]
	fn success_cases(words: Vec<&str>, expected: Vec<Vec<i32>>) {
		let result = super::Solution::palindrome_pairs(
			words.iter().map(|s| String::from(*s)).collect::<_>(),
		);

		assert!(expected.iter().all(|e| result.contains(e)));
	}
}
