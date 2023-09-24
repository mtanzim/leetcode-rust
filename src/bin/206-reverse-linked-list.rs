/*
 * @lc app=leetcode id=206 lang=rust
 *
 * [206] Reverse Linked List
 *
 * https://leetcode.com/problems/reverse-linked-list/description/
 *
 * algorithms
 * Easy (74.71%)
 * Likes:    19648
 * Dislikes: 357
 * Total Accepted:    3.4M
 * Total Submissions: 4.6M
 * Testcase Example:  '[1,2,3,4,5]'
 *
 * Given the head of a singly linked list, reverse the list, and return the
 * reversed list.
 *
 *
 * Example 1:
 *
 *
 * Input: head = [1,2,3,4,5]
 * Output: [5,4,3,2,1]
 *
 *
 * Example 2:
 *
 *
 * Input: head = [1,2]
 * Output: [2,1]
 *
 *
 * Example 3:
 *
 *
 * Input: head = []
 * Output: []
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the list is the range [0, 5000].
 * -5000 <= Node.val <= 5000
 *
 *
 *
 * Follow up: A linked list can be reversed either iteratively or recursively.
 * Could you implement both?
 *
 */

// @lc code=start
use std::vec;
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut stack = vec![];
        let mut cur_node = head.clone();
        loop {
            match cur_node {
                None => {
                    break;
                }
                Some(node) => {
                    let v = node.val;
                    stack.push(v);
                    cur_node = node.next;
                }
            }
        }
        stack.reverse();
        Self::generate_ll(stack)
    }
    // TODO: understand what is actually happening here with the borrow/move etc etc 🤷
    pub fn generate_ll(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut cur_node = &mut head;

        for &number in nums.iter() {
            let new_node = Some(Box::new(ListNode::new(number)));
            *cur_node = new_node;
            cur_node = &mut cur_node.as_mut().unwrap().next;
        }

        head
    }
}
// @lc code=end
struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn main() {
    let ll = Solution::generate_ll(vec![1, 2, 3, 4, 5]);
    println!("{:?}", ll);
    println!("{:?}", Solution::reverse_list(ll));
    // let res = Solution::reverse_list(ll);
}
#[cfg(test)]
mod tests_contains_dup {
    use crate::Solution;

    #[test]
    fn base() {
        let ll = Solution::generate_ll(vec![1, 2, 3, 4, 5]);
        let expect = Solution::generate_ll(vec![5, 4, 3, 2, 1]);
        let res = Solution::reverse_list(ll);
        assert_eq!(expect, res);
    }
}
