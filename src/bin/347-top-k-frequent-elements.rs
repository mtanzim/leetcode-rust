/*
 * @lc app=leetcode id=347 lang=rust
 *
 * [347] Top K Frequent Elements
 *
 * https://leetcode.com/problems/top-k-frequent-elements/description/
 *
 * algorithms
 * Medium (63.94%)
 * Likes:    15165
 * Dislikes: 538
 * Total Accepted:    1.6M
 * Total Submissions: 2.5M
 * Testcase Example:  '[1,1,1,2,2,3]\n2'
 *
 * Given an integer array nums and an integer k, return the k most frequent
 * elements. You may return the answer in any order.
 *
 * Example 1:
 * Input: nums = [1,1,1,2,2,3], k = 2
 * Output: [1,2]
 * Example 2:
 * Input: nums = [1], k = 1
 * Output: [1]
 *
 * Constraints:
 *
 * 1 <= nums.length <= 10^5
 * -10^4 <= nums[i] <= 10^4
 * k is in the range [1, the number of unique elements in the array].
 * It is guaranteed that the answer is unique.
 *
 * Follow up: Your algorithm's time complexity must be better than O(n log n),
 * where n is the array's size.
 */
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k_prime = k as usize;
        let mut scores = HashMap::new();
        for n in nums {
            let _ = match scores.get(&n) {
                Some(&found) => scores.insert(n, found + 1),
                _ => scores.insert(n, 1),
            };
        }
        let mut lst: Vec<(i32, i32)> = scores.into_iter().collect();
        lst.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));
        lst.iter().take(k_prime).map(|(k, _v)| *k).collect()
    }
}
// @lc code=end
struct Solution;
fn main() {
    let res = Solution::top_k_frequent(vec![1, 1, 3, 4], 1);
    println!("{:?}", res)
}

mod tests_top_k_freq_elems {

    #[test]
    fn with_k_1() {
        let res = super::Solution::top_k_frequent(vec![1, 1, 3, 4], 1);
        let expect = vec![1];
        assert_eq!(res, expect)
    }
    #[test]
    fn with_k_n() {
        let res = super::Solution::top_k_frequent(vec![1, 1, 3, 4], 3);
        let expect = vec![1, 4, 3];
        assert_eq!(res, expect)
    }
}
