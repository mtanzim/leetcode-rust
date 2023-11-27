/*
 * @lc app=leetcode id=23 lang=rust
 *
 * [23] Merge k Sorted Lists
 *
 * https://leetcode.com/problems/merge-k-sorted-lists/description/
 *
 * algorithms
 * Hard (51.33%)
 * Likes:    18621
 * Dislikes: 673
 * Total Accepted:    1.8M
 * Total Submissions: 3.5M
 * Testcase Example:  '[[1,4,5],[1,3,4],[2,6]]'
 *
 * You are given an array of k linked-lists lists, each linked-list is sorted
 * in ascending order.
 *
 * Merge all the linked-lists into one sorted linked-list and return it.
 *
 *
 * Example 1:
 *
 *
 * Input: lists = [[1,4,5],[1,3,4],[2,6]]
 * Output: [1,1,2,3,4,4,5,6]
 * Explanation: The linked-lists are:
 * [
 * ⁠ 1->4->5,
 * ⁠ 1->3->4,
 * ⁠ 2->6
 * ]
 * merging them into one sorted list:
 * 1->1->2->3->4->4->5->6
 *
 *
 * Example 2:
 *
 *
 * Input: lists = []
 * Output: []
 *
 *
 * Example 3:
 *
 *
 * Input: lists = [[]]
 * Output: []
 *
 *
 *
 * Constraints:
 *
 *
 * k == lists.length
 * 0 <= k <= 10^4
 * 0 <= lists[i].length <= 500
 * -10^4 <= lists[i][j] <= 10^4
 * lists[i] is sorted in ascending order.
 * The sum of lists[i].length will not exceed 10^4.
 *
 *
 */

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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.len() == 2 {
            return Self::merge_two_lists(
                lists.get(0).expect("cannot find first list").clone(),
                lists.get(1).expect("cannot find second list").clone(),
            );
        }
        if lists.len() == 1 {
            return lists.get(0).expect("cannot find first list").clone();
        }
        let mid = lists.len() / 2;
        let left_lists = lists[0..mid].to_vec();
        let right_lists = lists[mid..].to_vec();
        Self::merge_two_lists(
            Self::merge_k_lists(left_lists),
            Self::merge_k_lists(right_lists),
        )
    }
    fn merge_two_lists(
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
