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
    /// ### asked gippity: why do we use Rc and RefCell with a binary tree?
    ///Using `Rc` (Reference Counting) and `RefCell` with a binary tree is a common pattern in Rust when you need to create a tree structure that allows for shared ownership and interior mutability.
    ///1. **Shared Ownership (Rc)**:
    ///   
    ///   In a binary tree, a node may have multiple children (up to two in the case of a binary tree). Using `Rc`, you can have multiple references to the same node, allowing for shared ownership. This is useful because a node may be referenced by multiple parents (other nodes).
    ///
    ///2. **Interior Mutability (RefCell)**:
    ///   
    ///   `RefCell` allows you to achieve interior mutability, enabling you to modify the contents of a node even when you only have an immutable reference to it. This is important in a tree structure, where you might want to modify a node's children (left or right subtrees) while having an immutable reference to the node itself.
    ///
    ///Combining `Rc` and `RefCell`:
    ///
    ///- `Rc<RefCell<TreeNode>>` allows you to share a node among different parts of the program (multiple parents or references).
    ///- `RefCell` enables you to mutate the node's content (like modifying its children) even when you only have an immutable reference to it.
    ///
    ///Let's take an example: you might want to modify the left and right child of a node in a binary tree. You'd use `RefCell` to allow this interior mutability, and `Rc` to share the node across different parts of the tree without transferring ownership.
    ///
    ///```rust
    ///use std::cell::RefCell;
    ///use std::rc::Rc;
    ///#[derive(Debug)]
    ///struct TreeNode {
    ///    val: i32,
    ///    left: Option<Rc<RefCell<TreeNode>>>,
    ///    right: Option<Rc<RefCell<TreeNode>>>,
    ///}
    ///
    ///fn main() {
    ///    let left_child = Rc::new(RefCell::new(TreeNode {
    ///        val: 2,
    ///        left: None,
    ///        right: None,
    ///    }));
    ///
    ///    let right_child = Rc::new(RefCell::new(TreeNode {
    ///        val: 3,
    ///        left: None,
    ///        right: None,
    ///    }));
    ///
    ///    let root = Rc::new(RefCell::new(TreeNode {
    ///        val: 1,
    ///        left: Some(left_child.clone()),
    ///        right: Some(right_child.clone()),
    ///    }));
    ///
    ///    // Modify left child's value
    ///    left_child.borrow_mut().val = 10;
    ///
    ///    println!("{:?}", root);
    ///}
    ///```
    ///
    ///In this example, we create a binary tree with `Rc` and `RefCell`, allowing shared ownership and interior mutability to modify the values within the tree nodes.
    /// 
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
    println!("{:?}", Solution::max_depth(Some(root)));
}

mod tests {
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
        assert_eq!(Solution::max_depth(Some(root)), 2);
    }
}
