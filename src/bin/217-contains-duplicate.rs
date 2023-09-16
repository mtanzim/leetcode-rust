/*
 * @lc app=leetcode id=217 lang=rust
 *
 * [217] Contains Duplicate
 */

use std::collections::HashSet;

// @lc code=start
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut my_set = HashSet::new();
    if nums.len() == 0 {
        return false;
    }
    for n in &nums {
        if my_set.contains(n) {
            return true;
        }
        my_set.insert(n);
    }
    return false;
}
// @lc code=end

fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_contqains_dup() {
        let result = super::contains_duplicate(vec![1, 2, 3, 1]);
        assert_eq!(result, true);
    }
    #[test]
    fn basic_does_contain_dup() {
        let result = super::contains_duplicate(vec![1, 2, 3, ]);
        assert_eq!(result, false);
    }
    #[test]
    fn empty() {
        let result = super::contains_duplicate(vec![]);
        assert_eq!(result, false);
    }
}
