/*
 * @lc app=leetcode id=124 lang=rust
 *
 * [124] Binary Tree Maximum Path Sum
 *
 * https://leetcode.com/problems/binary-tree-maximum-path-sum/description/
 *
 * algorithms
 * Hard (39.64%)
 * Likes:    15807
 * Dislikes: 687
 * Total Accepted:    1.1M
 * Total Submissions: 2.8M
 * Testcase Example:  '[1,2,3]'
 *
 * A path in a binary tree is a sequence of nodes where each pair of adjacent
 * nodes in the sequence has an edge connecting them. A node can only appear in
 * the sequence at most once. Note that the path does not need to pass through
 * the root.
 *
 * The path sum of a path is the sum of the node's values in the path.
 *
 * Given the root of a binary tree, return the maximum path sum of any
 * non-empty path.
 *
 *
 * Example 1:
 *
 *
 * Input: root = [1,2,3]
 * Output: 6
 * Explanation: The optimal path is 2 -> 1 -> 3 with a path sum of 2 + 1 + 3 =
 * 6.
 *
 *
 * Example 2:
 *
 *
 * Input: root = [-10,9,20,null,null,15,7]
 * Output: 42
 * Explanation: The optimal path is 15 -> 20 -> 7 with a path sum of 15 + 20 +
 * 7 = 42.
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree is in the range [1, 3 * 10^4].
 * -1000 <= Node.val <= 1000
 *
 *
 */

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
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn traverse(node: Option<Rc<RefCell<TreeNode>>>, path_sum: Rc<RefCell<i32>>) -> i32 {
        match node {
            None => 0,
            Some(node_rc) => {
                let left_max = Self::traverse(node_rc.borrow().left.clone(), path_sum.clone());
                let right_max = Self::traverse(node_rc.borrow().right.clone(), path_sum.clone());
                let cur_v = node_rc.borrow().val;
                let rv_max = left_max.max(right_max).max(0) + cur_v;
                let local_max = (left_max + right_max + cur_v)
                    .max(cur_v + left_max)
                    .max(cur_v + right_max)
                    .max(cur_v);
                let mut cur_overall_max = path_sum.borrow_mut();
                if *cur_overall_max < local_max {
                    *cur_overall_max = local_max;
                }
                rv_max
            }
        }
    }
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let empty_tree = Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None,
        }));
        let root_val = root.clone().unwrap_or(empty_tree).borrow().val;
        let overall_max = Rc::new(RefCell::new(root_val));
        Self::traverse(root, overall_max.clone());
        let rv = overall_max.borrow();
        *rv
    }
}
// @lc code=end

struct Solution;
fn main() {
    println!("hello")
}

mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{Solution, TreeNode};

    #[test]
    fn basic1() {
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
        let rv = Solution::max_path_sum(Some(root));
        assert_eq!(rv, 6)
    }
}
