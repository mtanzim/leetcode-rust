/*
 * @lc app=leetcode id=268 lang=rust
 *
 * [268] Missing Number
 *
 * https://leetcode.com/problems/missing-number/description/
 *
 * algorithms
 * Easy (63.85%)
 * Likes:    10927
 * Dislikes: 3242
 * Total Accepted:    1.8M
 * Total Submissions: 2.7M
 * Testcase Example:  '[3,0,1]'
 *
 * Given an array nums containing n distinct numbers in the range [0, n],
 * return the only number in the range that is missing from the array.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [3,0,1]
 * Output: 2
 * Explanation: n = 3 since there are 3 numbers, so all numbers are in the
 * range [0,3]. 2 is the missing number in the range since it does not appear
 * in nums.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [0,1]
 * Output: 2
 * Explanation: n = 2 since there are 2 numbers, so all numbers are in the
 * range [0,2]. 2 is the missing number in the range since it does not appear
 * in nums.
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [9,6,4,2,3,5,7,0,1]
 * Output: 8
 * Explanation: n = 9 since there are 9 numbers, so all numbers are in the
 * range [0,9]. 8 is the missing number in the range since it does not appear
 * in nums.
 *
 *
 *
 * Constraints:
 *
 *
 * n == nums.length
 * 1 <= n <= 10^4
 * 0 <= nums[i] <= n
 * All the numbers of nums are unique.
 *
 *
 *
 * Follow up: Could you implement a solution using only O(1) extra space
 * complexity and O(n) runtime complexity?
 *
 */

// @lc code=start
impl Solution {
    pub fn missing_number_sum(nums: Vec<i32>) -> i32 {
        let range = 0..=nums.len() as i32;
        let expected_sum: i32 = range.sum();
        let sum: i32 = nums.iter().sum();
        expected_sum - sum
    }
    pub fn missing_number_sorted(nums: Vec<i32>) -> i32 {
        let mut num_sorted = nums.clone();
        num_sorted.sort();
        let filtered: Vec<i32> = num_sorted
            .iter()
            .enumerate()
            .filter(|(i, v)| *i as i32 != **v)
            .map(|(i, _)| i as i32)
            .collect();
        *(filtered.get(0).unwrap_or(&(nums.len() as i32)))
    }
}
// @lc code=end

struct Solution;

fn main() {
    let res = crate::Solution::missing_number_sorted(vec![3, 0, 1]);
    println!("{}", res);
}

mod tests {
    #[test]
    fn basic() {
        let res = crate::Solution::missing_number_sorted(vec![3, 0, 1]);
        let res_sum = crate::Solution::missing_number_sum(vec![3, 0, 1]);
        assert_eq!(res, 2);
        assert_eq!(res_sum, 2);
    }
    #[test]
    fn basic_2() {
        let res = crate::Solution::missing_number_sorted(vec![0, 1]);
        let res_sum = crate::Solution::missing_number_sorted(vec![0, 1]);
        assert_eq!(res, 2);
        assert_eq!(res_sum, 2);
    }
}
