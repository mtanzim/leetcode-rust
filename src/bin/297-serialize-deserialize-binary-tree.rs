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
        let arr_preorder = Rc::new(RefCell::new(vec![]));
        Codec::preorder(root.clone(), Rc::clone(&arr_preorder));
        let preorder_s = arr_preorder
            .borrow()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");
        preorder_s
    }

    fn preorder(node: Option<Rc<RefCell<TreeNode>>>, cur_vec: Rc<RefCell<Vec<String>>>) {
        match node {
            None => cur_vec.borrow_mut().push("N".to_string()),
            Some(node_rc) => {
                cur_vec.borrow_mut().push(node_rc.borrow().val.to_string());
                Self::preorder(node_rc.borrow().left.clone(), cur_vec.clone());
                Self::preorder(node_rc.borrow().right.clone(), cur_vec.clone());
            }
        }
    }

    // TODO: study vec vs slices
    fn preorder_invert(cur_vec: Vec<String>) -> (Option<Rc<RefCell<TreeNode>>>, Vec<String>) {
        if cur_vec.len() == 0 {
            return (None, cur_vec);
        }

        let cur_token = cur_vec.get(0);
        match cur_token {
            None => (None, cur_vec),
            Some(token) => {
                if token == "N" {
                    return (None, cur_vec[1..].to_vec());
                }
                let node_val = token.parse::<i32>().expect("cannot parse numbered token");
                let next_vec = if cur_vec.len() > 0 {
                    cur_vec[1..].to_vec()
                } else {
                    vec![]
                };
                let (left_node, left_updated_vec) = Self::preorder_invert(next_vec);
                let (right_node, right_updated_vec) = Self::preorder_invert(left_updated_vec);
                (
                    Some(Rc::new(RefCell::new(TreeNode {
                        val: node_val,
                        left: left_node,
                        right: right_node,
                    }))),
                    right_updated_vec,
                )
            }
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let preorder_vec: Vec<String> = data.split(",").map(|token| token.to_string()).collect();
        Self::preorder_invert(preorder_vec).0
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
    fn preorder() {
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
        let arr_preorder = Rc::new(RefCell::new(vec![]));
        Codec::preorder(Some(Rc::clone(&root)), Rc::clone(&arr_preorder));
        println!("preorder: {:?}", arr_preorder);
        assert_eq!(
            arr_preorder.borrow().as_ref(),
            vec!["2", "1", "N", "N", "3", "N", "N"]
        );
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
        assert_eq!(c.serialize(Some(root.clone())), "2,1,N,N,3,N,N".to_string());
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
        let serialized = "2,1,N,N,3,N,N".to_string();
        let tree = Some(root);
        let c = Codec::new();
        assert_eq!(c.deserialize(serialized), tree);
    }
}
