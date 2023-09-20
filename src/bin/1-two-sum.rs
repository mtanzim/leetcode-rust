/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */


// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm = HashMap::new();
        for (index, element) in nums.iter().enumerate() {
            let diff = target - element;
            let val = hm.get(element);
            match val {
                None => {}
                Some(other_index) => return vec![*other_index, index as i32],
            }
            hm.insert(diff, index as i32);
        }
        return vec![];
    }
}
// @lc code=end
struct Solution;

fn main() {
    let res = Solution::two_sum(vec![2, 7, 11, 15], 9);
    println!("{:?}", res)
}

mod tests_two_sum {

    #[test]
    fn basic() {
        let res = super::Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(vec![0, 1], res);
    }
    #[test]
    fn basic_second() {
        let res = super::Solution::two_sum(vec![3, 2, 4], 6);
        assert_eq!(vec![1, 2], res);
    }
    #[test]
    fn basic_third() {
        let res = super::Solution::two_sum(vec![3, 3], 6);
        assert_eq!(vec![0, 1], res);
    }
}
