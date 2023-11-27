/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
 *
 * https://leetcode.com/problems/merge-two-sorted-lists/description/
 *
 * algorithms
 * Easy (63.42%)
 * Likes:    20543
 * Dislikes: 1918
 * Total Accepted:    3.7M
 * Total Submissions: 5.8M
 * Testcase Example:  '[1,2,4]\n[1,3,4]'
 *
 * You are given the heads of two sorted linked lists list1 and list2.
 *
 * Merge the two lists into one sorted list. The list should be made by
 * splicing together the nodes of the first two lists.
 *
 * Return the head of the merged linked list.
 *
 *
 * Example 1:
 *
 *
 * Input: list1 = [1,2,4], list2 = [1,3,4]
 * Output: [1,1,2,3,4,4]
 *
 *
 * Example 2:
 *
 *
 * Input: list1 = [], list2 = []
 * Output: []
 *
 *
 * Example 3:
 *
 *
 * Input: list1 = [], list2 = [0]
 * Output: [0]
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in both lists is in the range [0, 50].
 * -100 <= Node.val <= 100
 * Both list1 and list2 are sorted in non-decreasing order.
 *
 *
 */

// Definition for singly-linked list.
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

// @lc code=start

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(p1), None) => Some(p1.clone()),
            (None, Some(p2)) => Some(p2.clone()),
            (Some(p1), Some(p2)) => {
                if p1.val < p2.val {
                    return Some(Box::new(ListNode {
                        val: p1.val,
                        next: Self::merge_two_lists(p1.next.clone(), Some(p2.clone())),
                    }));
                }
                Some(Box::new(ListNode {
                    val: p2.val,
                    next: Self::merge_two_lists(Some(p1.clone()), p2.next.clone()),
                }))
            }
        }
    }
}
// @lc code=end

struct Solution;
fn main() {
    println!("hello")
}

mod tests {
    #[test]
    fn basic1() {
        assert_eq!(1, 1)
    }
}
