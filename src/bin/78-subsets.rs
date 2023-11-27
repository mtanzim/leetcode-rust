/*
 * @lc app=leetcode id=78 lang=rust
 *
 * [78] Subsets
 *
 * https://leetcode.com/problems/subsets/description/
 *
 * algorithms
 * Medium (76.47%)
 * Likes:    16169
 * Dislikes: 241
 * Total Accepted:    1.6M
 * Total Submissions: 2.2M
 * Testcase Example:  '[1,2,3]'
 *
 * Given an integer array nums of unique elements, return all possible subsets
 * (the power set).
 *
 * The solution set must not contain duplicate subsets. Return the solution in
 * any order.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,2,3]
 * Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [0]
 * Output: [[],[0]]
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10
 * -10 <= nums[i] <= 10
 * All the numbers of nums are unique.
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 0 {
            return vec![vec![]];
        }

        let head = nums[0];
        let tail = &nums[1..];
        let tail_vecs = Self::subsets(tail.to_vec());
        let tail_vecs_with_head: Vec<Vec<i32>> = tail_vecs
            .iter()
            .map(|v| {
                let mut new_vec = vec![head];
                new_vec.extend(v);
                new_vec
            })
            .collect();
        tail_vecs
            .iter()
            .cloned()
            .chain(tail_vecs_with_head.iter().cloned())
            .collect()
    }
}
// @lc code=end
struct Solution {}
fn main() {
    println!("hello")
}

mod tests {
    use crate::Solution;

    #[test]
    fn base_case() {
        let d = vec![1];
        let expect = vec![vec![], vec![1]];
        assert_eq!(Solution::subsets(d), expect)
    }

    #[test]
    fn base_case_plus_1() {
        let d = vec![1, 2];
        let expect = vec![vec![], vec![2], vec![1], vec![1, 2]];
        assert_eq!(Solution::subsets(d), expect)
    }

    #[test]
    fn basic1() {
        let d = vec![1, 2, 3];
        let expect = vec![
            vec![],
            vec![3],
            vec![2],
            vec![2, 3],
            vec![1],
            vec![1, 3],
            vec![1, 2],
            vec![1, 2, 3],
        ];
        assert_eq!(Solution::subsets(d), expect)
    }
}
