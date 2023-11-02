/*
 * @lc app=leetcode id=110 lang=rust
 *
 * [110] Balanced Binary Tree
 *
 * https://leetcode.com/problems/balanced-binary-tree/description/
 *
 * algorithms
 * Easy (50.34%)
 * Likes:    10040
 * Dislikes: 586
 * Total Accepted:    1.3M
 * Total Submissions: 2.5M
 * Testcase Example:  '[3,9,20,null,null,15,7]'
 *
 * Given a binary tree, determine if it is height-balanced.
 *
 *
 * Example 1:
 *
 *
 * Input: root = [3,9,20,null,null,15,7]
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: root = [1,2,2,3,3,null,null,4,4]
 * Output: false
 *
 *
 * Example 3:
 *
 *
 * Input: root = []
 * Output: true
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree is in the range [0, 5000].
 * -10^4 <= Node.val <= 10^4
 *
 *
 */

// @lc code=start

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn height(node: Option<Rc<RefCell<TreeNode>>>, cur_height: i32) -> i32 {
        match node {
            None => cur_height,
            Some(node_rc) => {
                let next_height = cur_height + 1;
                let next_node = node_rc.borrow();
                let lh = Self::height(next_node.left.clone(), next_height);
                let rh = Self::height(next_node.right.clone(), next_height);
                lh.max(rh)
            }
        }
    }
    fn check_balanced(node: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match node {
            None => true,
            Some(node_rc) => {
                let lv = Self::height(node_rc.borrow().left.clone(), 0);
                let rv = Self::height(node_rc.borrow().right.clone(), 0);
                let delta = (lv - rv).abs();
                delta < 2
                    && Self::check_balanced(node_rc.borrow().left.clone())
                    && Self::check_balanced(node_rc.borrow().right.clone())
            }
        }
    }
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::check_balanced(root.clone())
    }
}
// @lc code=end

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

struct Solution;
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
    println!("{:?}", Solution::is_balanced(Some(root)));
}

mod tests_top_k_freq_elems {
    use std::{cell::RefCell, rc::Rc};

    use crate::{Solution, TreeNode};

    #[test]
    fn basic() {
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
        assert_eq!(true, Solution::is_balanced(Some(root)))
    }
}
