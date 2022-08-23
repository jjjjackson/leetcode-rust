// https://leetcode.com/problems/palindrome-linked-list/submissions/

use crate::helpers::list::ListNode;
use crate::solutions::Solution;

// Create Reverse Node 超慢...
impl Solution {
	pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
		let mut length = 0;
		let mut temp_head = &head;
		let mut rev_link: Option<Box<ListNode>> = None;
		while let Some(h) = temp_head {
			length += 1;
			temp_head = &h.next;
			rev_link = Some(Box::new(ListNode {
				val: h.val,
				next: rev_link,
			}));
		}

		temp_head = &head;
		let mut temp_rev_link = &rev_link;
		for _ in 0..length / 2 {
			if let (Some(h), Some(r)) = (temp_head, temp_rev_link) {
				if h.val != r.val {
					return false;
				} else {
					temp_head = &h.next;
					temp_rev_link = &r.next;
				}
			}
		}

		true
	}
}

// 93.5%
// impl Solution {
// 	pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
// 		let mut nums: Vec<i32> = vec![];

// 		while let Some(h) = head {
// 			nums.push(h.val);
// 			head = h.next;
// 		}

// 		for i in 0..nums.len(){
// 			if nums[i] != nums[nums.len()-i-1] {
// 				return false;
// 			}
// 		}

// 		true
// 	}
// }
