/*
 * @lc app=leetcode id=230 lang=rust
 *
 * [230] Kth Smallest Element in a BST
 *
 * https://leetcode.com/problems/kth-smallest-element-in-a-bst/description/
 *
 * algorithms
 * Medium (71.20%)
 * Likes:    10665
 * Dislikes: 195
 * Total Accepted:    1.2M
 * Total Submissions: 1.7M
 * Testcase Example:  '[3,1,4,null,2]\n1'
 *
 * Given the root of a binary search tree, and an integer k, return the k^th
 * smallest value (1-indexed) of all the values of the nodes in the tree.
 *
 *
 * Example 1:
 *
 *
 * Input: root = [3,1,4,null,2], k = 1
 * Output: 1
 *
 *
 * Example 2:
 *
 *
 * Input: root = [5,3,6,2,4,null,null,1], k = 3
 * Output: 3
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree is n.
 * 1 <= k <= n <= 10^4
 * 0 <= Node.val <= 10^4
 *
 *
 *
 * Follow up: If the BST is modified often (i.e., we can do insert and delete
 * operations) and you need to find the kth smallest frequently, how would you
 * optimize?
 *
 */

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
// Definition for a binary tree node.

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// ### Note the `clone()` of the `left` and `right` child here; thanks again gippity
    /// In Rust, the clone method typically creates a deep copy of the data, including its contents, for types that implement the Clone trait. However, this behavior can vary depending on the type being cloned and how Clone is implemented for that type.
    ///
    /// When you call clone on a type that implements Clone, it usually creates a new instance of that type with its own, separate memory allocation (i.e., a deep copy). This means that the cloned object is distinct from the original and modifying one does not affect the other.
    ///
    /// However, it's essential to understand that Rust types control how Clone behaves. Some types might perform a shallow copy (like copying a reference or a pointer) if a deep copy is not feasible or efficient.
    ///
    /// For types like Rc, Arc, and RefCell, the clone method usually creates a new reference (a new pointer), not a deep copy of the data the pointer points to. It increases the reference count, making multiple parts of the program share the same underlying data. This is known as "cloning the reference" or "cloning the pointer."
    ///
    /// So, in the case of `node.borrow().left.clone()`, it typically creates a new reference to the left child, not a deep copy of the entire heap content. It's effectively cloning the pointer (reference) to the left child, allowing multiple parts of the program to share access to the same left child node.
    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, stack: &mut Vec<i32>) {
        match root {
            None => {
                return;
            }
            Some(node) => {
                Self::traverse(node.borrow().left.clone(), stack);
                stack.push(node.borrow().val);
                Self::traverse(node.borrow().right.clone(), stack);
            }
        }
    }
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack = vec![];
        Self::traverse(root, &mut stack);
        return stack[(k - 1) as usize];
    }
}
// @lc code=end

struct Solution;
fn main() {
    let res = Solution::kth_smallest(None, 0);
    println!("{:?}", res)
}

mod tests_kth_smallest_bst {
    use std::{cell::RefCell, rc::Rc};

    use crate::{Solution, TreeNode};

    #[test]
    fn base() {
        let left_child = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }));

        let right_child = Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        }));

        let root = Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(left_child.clone()),
            right: Some(right_child.clone()),
        }));
        assert_eq!(Solution::kth_smallest(Some(root), 1), 1);
    }
}
