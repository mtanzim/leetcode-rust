/*
 * @lc app=leetcode id=543 lang=rust
 *
 * [543] Diameter of Binary Tree
 *
 * https://leetcode.com/problems/diameter-of-binary-tree/description/
 *
 * algorithms
 * Easy (57.93%)
 * Likes:    12671
 * Dislikes: 812
 * Total Accepted:    1.2M
 * Total Submissions: 2.1M
 * Testcase Example:  '[1,2,3,4,5]'
 *
 * Given the root of a binary tree, return the length of the diameter of the
 * tree.
 *
 * The diameter of a binary tree is the length of the longest path between any
 * two nodes in a tree. This path may or may not pass through the root.
 *
 * The length of a path between two nodes is represented by the number of edges
 * between them.
 *
 *
 * Example 1:
 *
 *
 * Input: root = [1,2,3,4,5]
 * Output: 3
 * Explanation: 3 is the length of the path [4,2,1,3] or [5,2,1,3].
 *
 *
 * Example 2:
 *
 *
 * Input: root = [1,2]
 * Output: 1
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree is in the range [1, 10^4].
 * -100 <= Node.val <= 100
 *
 *
 */

// @lc code=start

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn traverse(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match node {
            None => (-1, 0),
            Some(node_rc) => {
                let (lh_p, ld) = Self::traverse(node_rc.borrow().left.clone());
                let (rh_p, rd) = Self::traverse(node_rc.borrow().right.clone());
                let lh = lh_p + 1;
                let rh = rh_p + 1;
                let cur_d = lh + rh;
                return (lh.max(rh), cur_d.max(ld).max(rd));
            }
        }
    }
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::traverse(root).1
    }
}

// @lc code=end
struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
fn main() {
    let left_child = Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: None,
        right: None,
    }));

    let right_child = Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: None,
        right: None,
    }));

    let root = Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(left_child.clone()),
        right: Some(right_child.clone()),
    }));
    println!("{:?}", Solution::diameter_of_binary_tree(Some(root)));
}
