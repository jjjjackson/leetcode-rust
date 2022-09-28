use crate::helpers::list::ListNode;
use crate::solutions::Solution;

impl Solution {
	pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
		Self::remove_nth_from_end_helper(head, n).0
	}

	pub fn remove_nth_from_end_helper(
		head: Option<Box<ListNode>>,
		n: i32,
	) -> (Option<Box<ListNode>>, i32) {
		match head {
			None => (None, 1),
			Some(mut node) => {
				let (prev, num) = Self::remove_nth_from_end_helper(node.next.take(), n);
				if num == n {
					(prev, num + 1)
				} else {
					node.next = prev;
					(Some(node), num + 1)
				}
			}
		}
	}
}
