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
    /// #### Gippity explains its own `generate_ll` function
    /// Absolutely, let's dive into the moves and borrows in the given Rust code more clearly.
    /// 1. **Moves**:
    /// In Rust, ownership and moves are important concepts for memory safety. When you move a value, you transfer ownership from one variable to another, and the original variable can no longer be used.
    /// ```rust
    /// let mut head = None;
    /// ```
    /// Here, `None` is moved into `head`. `head` is now the owner of this `Option<Box<ListNode>>` value.
    /// ```rust
    /// let new_node = Some(Box::new(ListNode::new(number)));
    /// ```
    /// In this line, a new `Option<Box<ListNode>>` is created, and `new_node` takes ownership of this value.
    /// ```rust
    /// *cur_node = new_node;
    /// ```
    /// The `new_node` is moved into the `cur_node` which is a mutable reference to `head`. `cur_node` takes ownership of the `new_node`, and now `cur_node` is the owner of the `Option<Box<ListNode>>`.
    /// 2. **Borrows**:
    /// Borrows in Rust refer to references or temporary permissions to use a value, but they don't own the value. Borrowing helps ensure that while some part of the code is using a value, it cannot be modified or moved elsewhere.
    /// ```rust
    /// let mut cur_node = &mut head;
    /// ```
    /// Here, a mutable reference `&mut head` is created and assigned to `cur_node`. This means that `cur_node` is temporarily borrowing a mutable reference to the value owned by `head`.
    /// ```rust
    /// cur_node = &mut cur_node.as_mut().unwrap().next;
    /// ```
    /// This line updates `cur_node` to point to the `.next` of the previous node, effectively "moving" the reference to the next node in the linked list. This is a mutable borrow, meaning we're temporarily modifying where `cur_node` is pointing to.
    /// Understanding moves and borrows is crucial in Rust to ensure memory safety and prevent common issues like use-after-move or use-after-free. Rust's ownership system enforces strict rules around these concepts to guarantee memory safety at compile time.
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
