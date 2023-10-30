/*
 * @lc app=leetcode id=704 lang=rust
 *
 * [704] Binary Search
 *
 * https://leetcode.com/problems/binary-search/description/
 *
 * algorithms
 * Easy (56.61%)
 * Likes:    11032
 * Dislikes: 222
 * Total Accepted:    2.1M
 * Total Submissions: 3.6M
 * Testcase Example:  '[-1,0,3,5,9,12]\n9'
 *
 * Given an array of integers nums which is sorted in ascending order, and an
 * integer target, write a function to search target in nums. If target exists,
 * then return its index. Otherwise, return -1.
 *
 * You must write an algorithm with O(log n) runtime complexity.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [-1,0,3,5,9,12], target = 9
 * Output: 4
 * Explanation: 9 exists in nums and its index is 4
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [-1,0,3,5,9,12], target = 2
 * Output: -1
 * Explanation: 2 does not exist in nums so return -1
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^4
 * -10^4 < nums[i], target < 10^4
 * All the integers in nums are unique.
 * nums is sorted in ascending order.
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l: i32 = 0;
        let mut r:i32 = (nums.len() - 1) as i32;
        while l <= r {
            let mid:i32 = (l + r) / 2;
            // TODO: how to avoid this typecast?
            let val = nums[mid as usize];
            if target > val {
                l = mid + 1
            } else if target < val {
                r = mid - 1;
            } else {
                return mid as i32;
            }
        }
        -1
    }
}
// @lc code=end
struct Solution;

mod tests {
    use crate::Solution;

    #[test]
    fn basic() {
        let nums = vec![1, 2, 3, 4];
        let target = 3;
        assert_eq!(Solution::search(nums, target), 2);
    }
    #[test]
    fn basic_2() {
        let nums = vec![1, 2, 3, 4];
        let target = 4;
        assert_eq!(Solution::search(nums, target), 3);
    }
    #[test]
    fn basic_3() {
        let nums = vec![1, 2, 3, 4];
        let target = 1;
        assert_eq!(Solution::search(nums, target), 0);
    }
    #[test]
    fn basic_4() {
      let nums = vec![1, 2, 3, 4];
      let target = 44;
      assert_eq!(Solution::search(nums, target), -1);
  }
  #[test]
  fn basic_5() {
    let nums = vec![5];
    let target = -5;
    assert_eq!(Solution::search(nums, target), -1);
}
}
