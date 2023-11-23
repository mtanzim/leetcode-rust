/*
 * @lc app=leetcode id=297 lang=rust
 *
 * [297] Serialize and Deserialize Binary Tree
 *
 * https://leetcode.com/problems/serialize-and-deserialize-binary-tree/description/
 *
 * algorithms
 * Hard (56.13%)
 * Likes:    9681
 * Dislikes: 354
 * Total Accepted:    805.4K
 * Total Submissions: 1.4M
 * Testcase Example:  '[1,2,3,null,null,4,5]'
 *
 * Serialization is the process of converting a data structure or object into a
 * sequence of bits so that it can be stored in a file or memory buffer, or
 * transmitted across a network connection link to be reconstructed later in
 * the same or another computer environment.
 *
 * Design an algorithm to serialize and deserialize a binary tree. There is no
 * restriction on how your serialization/deserialization algorithm should work.
 * You just need to ensure that a binary tree can be serialized to a string and
 * this string can be deserialized to the original tree structure.
 *
 * Clarification: The input/output format is the same as how LeetCode
 * serializes a binary tree. You do not necessarily need to follow this format,
 * so please be creative and come up with different approaches yourself.
 *
 *
 * Example 1:
 *
 *
 * Input: root = [1,2,3,null,null,4,5]
 * Output: [1,2,3,null,null,4,5]
 *
 *
 * Example 2:
 *
 *
 * Input: root = []
 * Output: []
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree is in the range [0, 10^4].
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

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
// TODO: prefer Rc::clone() as it makes it more obvious that we are cloning the "reference"
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let arr_inorder = Rc::new(RefCell::new(vec![]));
        let arr_preorder = Rc::new(RefCell::new(vec![]));
        Codec::in_order(root.clone(), Rc::clone(&arr_inorder));
        Codec::pre_order(root.clone(), Rc::clone(&arr_preorder));
        let inorder_s = arr_inorder
            .borrow()
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let preorder_s = arr_preorder
            .borrow()
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");
        format!("inorder:{}\npreorder:{}", inorder_s, preorder_s)
    }

    fn in_order(node: Option<Rc<RefCell<TreeNode>>>, cur_vec: Rc<RefCell<Vec<i32>>>) {
        match node {
            None => return,
            Some(node_rc) => {
                Self::in_order(node_rc.borrow().left.clone(), cur_vec.clone());
                cur_vec.borrow_mut().push(node_rc.borrow().val);
                Self::in_order(node_rc.borrow().right.clone(), cur_vec.clone());
            }
        }
    }

    fn pre_order(node: Option<Rc<RefCell<TreeNode>>>, cur_vec: Rc<RefCell<Vec<i32>>>) {
        match node {
            None => return,
            Some(node_rc) => {
                cur_vec.borrow_mut().push(node_rc.borrow().val);
                Self::pre_order(node_rc.borrow().left.clone(), cur_vec.clone());
                Self::pre_order(node_rc.borrow().right.clone(), cur_vec.clone());
            }
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let splits: Vec<&str> = data.split("\n").collect();
        let ineorder_raw = *splits.get(0).expect("failed to parse inorder raw");
        let preorder_raw = *splits.get(1).expect("failed to parse preorder raw");
        let inorder_clean = ineorder_raw.replace("inorder:", "");
        let preorder_clean = preorder_raw.replace("preorder:", "");

        if preorder_clean.len() == 0 || inorder_clean.len() == 0 {
            return None;
        }

        let inorder_vec: Vec<i32> = inorder_clean
            .split(",")
            .map(|token| token.parse::<i32>().expect("failed to parse inorder token"))
            .collect();
        let preorder_vec: Vec<i32> = preorder_clean
            .split(",")
            .map(|token| token.parse::<i32>().expect("failed to parse inorder token"))
            .collect();
        Self::build_tree(preorder_vec, inorder_vec)
    }

    // TODO: this assumes all values are unique and thus the solution does not always work
    // ie: [3,2,4,3]
    fn buiild_tree_traverse(
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
        let (left_tree, left_updated_pre_order) =
            Self::buiild_tree_traverse(next_pre_order.clone(), left_inorder.clone());
        let right_inorder: Vec<i32> = inorder.clone()[inorder_cut + 1..].to_vec();
        let (right_tree, right_updated_pre_order) =
            Self::buiild_tree_traverse(left_updated_pre_order.clone(), right_inorder.clone());
        let node = TreeNode {
            val: root_val,
            left: left_tree,
            right: right_tree,
        };
        let tree = Some(Rc::new(RefCell::new(node)));
        (tree, right_updated_pre_order)
    }
    // TODO: preorder needs to be globally mutable?
    fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::buiild_tree_traverse(preorder, inorder).0
    }
}

struct Codec;
// @lc code=end

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */

fn main() {
    println!("hello")
}

mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{Codec, TreeNode};

    #[test]
    fn inorder_preorder() {
        let left_child = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }));

        let right_child = Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }));

        let root = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(left_child.clone()),
            right: Some(right_child.clone()),
        }));
        let arr_inorder = Rc::new(RefCell::new(vec![]));
        let arr_preorder = Rc::new(RefCell::new(vec![]));
        Codec::in_order(Some(Rc::clone(&root)), Rc::clone(&arr_inorder));
        Codec::pre_order(Some(Rc::clone(&root)), Rc::clone(&arr_preorder));
        println!("inorder: {:?}", arr_inorder);
        println!("preorder: {:?}", arr_preorder);
        assert_eq!(arr_inorder.borrow().as_ref(), vec![1, 2, 3]);
        assert_eq!(arr_preorder.borrow().as_ref(), vec![2, 1, 3]);
    }

    #[test]
    fn serialize() {
        let left_child = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }));

        let right_child = Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }));

        let root = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(left_child.clone()),
            right: Some(right_child.clone()),
        }));

        let c = Codec::new();
        assert_eq!(
            c.serialize(Some(root.clone())),
            "inorder:1,2,3\npreorder:2,1,3".to_string()
        );
    }

    #[test]
    fn deserialize() {
        let left_child = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }));

        let right_child = Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }));

        let root = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(left_child.clone()),
            right: Some(right_child.clone()),
        }));
        let serialized = "inorder:1,2,3\npreorder:2,1,3".to_string();
        let tree = Some(root);
        let c = Codec::new();
        assert_eq!(c.deserialize(serialized), tree);
    }
}
