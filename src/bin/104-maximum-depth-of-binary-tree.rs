/*
 * @lc app=leetcode id=104 lang=rust
 *
 * [104] Maximum Depth of Binary Tree
 *
 * https://leetcode.com/problems/maximum-depth-of-binary-tree/description/
 *
 * algorithms
 * Easy (74.49%)
 * Likes:    11868
 * Dislikes: 193
 * Total Accepted:    2.6M
 * Total Submissions: 3.5M
 * Testcase Example:  '[3,9,20,null,null,15,7]'
 *
 * Given the root of a binary tree, return its maximum depth.
 *
 * A binary tree's maximum depth is the number of nodes along the longest path
 * from the root node down to the farthest leaf node.
 *
 *
 * Example 1:
 *
 *
 * Input: root = [3,9,20,null,null,15,7]
 * Output: 3
 *
 *
 * Example 2:
 *
 *
 * Input: root = [1,null,2]
 * Output: 2
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree is in the range [0, 10^4].
 * -100 <= Node.val <= 100
 *
 *
 */

// @lc code=start

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn traverse(node: Option<Rc<RefCell<TreeNode>>>, cur_height: i32) -> i32 {
        match node {
            None => cur_height,
            Some(node_rc) => {
                let next_height = cur_height + 1;
                let next_node = node_rc.borrow();
                let lh = Self::traverse(next_node.left.clone(), next_height);
                let rh = Self::traverse(next_node.right.clone(), next_height);
                lh.max(rh)
            }
        }
    }
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::traverse(root, 0)
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
    // let res = Solution::max_depth();
    println!("{:?}", 42)
}

mod tests {
    #[test]
    fn basic() {
        assert_eq!(true, true);
    }
}
