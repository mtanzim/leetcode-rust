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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    fn get_heights(
        node: Option<Rc<RefCell<TreeNode>>>,
        cur_height: i32,
        tracker: &mut HashMap<i32, i32>,
    ) -> i32 {
        match node {
            None => cur_height,
            Some(node_rc) => {
                let next_height = cur_height + 1;
                let next_node = node_rc.borrow();
                let lh = Self::get_heights(next_node.left.clone(), next_height, tracker);
                let rh = Self::get_heights(next_node.right.clone(), next_height, tracker);
                let tree_h = lh.max(rh);
                let node_h = tree_h - cur_height;
                tracker.insert(node_rc.borrow().val, node_h);
                tree_h
            }
        }
    }
    fn check_balanced(node: Option<Rc<RefCell<TreeNode>>>, height_map: HashMap<i32, i32>) -> bool {
        match node {
            None => true,
            Some(node_rc) => {
                let lc = node_rc.borrow().left.clone();
                let rc = node_rc.borrow().right.clone();
                let lv = match lc {
                    None => 0,
                    Some(node) => node.borrow().val,
                };
                let rv = match rc {
                    None => 0,
                    Some(node) => node.borrow().val,
                };
                let delta = (lv - rv).abs();
                delta < 2
                    && Self::check_balanced(node_rc.borrow().left.clone(), height_map.clone())
                    && Self::check_balanced(node_rc.borrow().right.clone(), height_map.clone())
            }
        }
    }
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut height_map = HashMap::new();
        Self::get_heights(root.clone(), 0, &mut height_map);
        println!("{:?}", height_map);
        Self::check_balanced(root.clone(), height_map)
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
    println!("{:?}", root);
    println!("{:?}", Solution::is_balanced(Some(root)));
}

mod tests_top_k_freq_elems {

    #[test]
    fn basic() {
        assert_eq!(true, true)
    }
}
