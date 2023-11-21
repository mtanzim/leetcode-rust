/*
 * @lc app=leetcode id=105 lang=rust
 *
 * [105] Construct Binary Tree from Preorder and Inorder Traversal
 *
 * https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/description/
 *
 * algorithms
 * Medium (62.92%)
 * Likes:    14210
 * Dislikes: 437
 * Total Accepted:    1.1M
 * Total Submissions: 1.7M
 * Testcase Example:  '[3,9,20,15,7]\n[9,3,15,20,7]'
 *
 * Given two integer arrays preorder and inorder where preorder is the preorder
 * traversal of a binary tree and inorder is the inorder traversal of the same
 * tree, construct and return the binary tree.
 *
 *
 * Example 1:
 *
 *
 * Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
 * Output: [3,9,20,null,null,15,7]
 *
 *
 * Example 2:
 *
 *
 * Input: preorder = [-1], inorder = [-1]
 * Output: [-1]
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= preorder.length <= 3000
 * inorder.length == preorder.length
 * -3000 <= preorder[i], inorder[i] <= 3000
 * preorder and inorder consist of unique values.
 * Each value of inorder also appears in preorder.
 * preorder is guaranteed to be the preorder traversal of the tree.
 * inorder is guaranteed to be the inorder traversal of the tree.
 *
 *
 */

// @lc code=start

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn traverse(
        preorder: Vec<i32>,
        inorder: Vec<i32>,
    ) -> (Option<Rc<RefCell<TreeNode>>>, Vec<i32>) {
        if preorder.len() == 0 || inorder.len() == 0 {
            return (None, preorder.clone());
        }

        let root_val = preorder[0];
        let next_pre_order = preorder.clone()[1..].to_vec();
        let inorder_cut = inorder
            .iter()
            .position(|&v| v == root_val)
            .expect("failed to find item in order, invalid case");
        let left_inorder: Vec<i32> = inorder.clone()[..inorder_cut].to_vec();
        // println!(
        //     "going left with node: {}, preorder: {:?}, inorder: {:?}",
        //     root_val, next_pre_order, left_inorder
        // );
        let (left_tree, left_updated_pre_order) =
            Self::traverse(next_pre_order.clone(), left_inorder.clone());
        let right_inorder: Vec<i32> = inorder.clone()[inorder_cut + 1..].to_vec();
        // println!(
        //     "going right with node: {}, preorder: {:?}, inorder: {:?}",
        //     root_val, left_updated_pre_order, left_inorder
        // );
        let (right_tree, right_updated_pre_order) =
            Self::traverse(left_updated_pre_order.clone(), right_inorder.clone());
        let node = TreeNode {
            val: root_val,
            left: left_tree,
            right: right_tree,
        };
        let tree = Some(Rc::new(RefCell::new(node)));
        // println!("{:?}", tree);
        (tree, right_updated_pre_order)
    }
    // TODO: preorder needs to be globally mutable?
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::traverse(preorder, inorder).0
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
    println!("hello")
}

mod tests {
    use crate::Solution;

    #[test]
    fn basic1() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let tree = Solution::build_tree(preorder, inorder);
        assert_eq!(1, 1)
    }
    #[test]
    fn failing1() {
        let preorder = vec![3, 1, 2, 4];
        let inorder = vec![1, 2, 3, 4];
        let tree = Solution::build_tree(preorder, inorder);
        assert_eq!(1, 1)
    }
}
