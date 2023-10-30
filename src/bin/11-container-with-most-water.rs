/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 *
 * https://leetcode.com/problems/container-with-most-water/description/
 *
 * algorithms
 * Medium (54.14%)
 * Likes:    27105
 * Dislikes: 1491
 * Total Accepted:    2.5M
 * Total Submissions: 4.5M
 * Testcase Example:  '[1,8,6,2,5,4,8,3,7]'
 *
 * You are given an integer array height of length n. There are n vertical
 * lines drawn such that the two endpoints of the i^th line are (i, 0) and (i,
 * height[i]).
 *
 * Find two lines that together with the x-axis form a container, such that the
 * container contains the most water.
 *
 * Return the maximum amount of water a container can store.
 *
 * Notice that you may not slant the container.
 *
 *
 * Example 1:
 *
 *
 * Input: height = [1,8,6,2,5,4,8,3,7]
 * Output: 49
 * Explanation: The above vertical lines are represented by array
 * [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the
 * container can contain is 49.
 *
 *
 * Example 2:
 *
 *
 * Input: height = [1,1]
 * Output: 1
 *
 *
 *
 * Constraints:
 *
 *
 * n == height.length
 * 2 <= n <= 10^5
 * 0 <= height[i] <= 10^4
 *
 *
 */

// @lc code=start
impl Solution {
    // brute force solution
    pub fn max_area_brute(height: Vec<i32>) -> i32 {
        let mut max_area: i32 = 0;
        for l in 0..height.len() {
            for r in l + 1..height.len() {
                let (h1, h2) = (height[l], height[r]);
                let cur_area: i32 = (r as i32 - l as i32) * std::cmp::min(h1, h2);
                max_area = std::cmp::max(max_area, cur_area);
            }
        }
        return max_area;
    }
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area: i32 = 0;
        let mut l: i32 = 0;
        let mut r: i32 = height.len() as i32 - 1;
        while l < r && l >= 0 && r >= 0 {
            let (lh, rh) = (height[l as usize], height[r as usize]);
            let cur_area: i32 = (r as i32 - l as i32) * std::cmp::min(lh, rh);
            max_area = std::cmp::max(max_area, cur_area);
            if lh > rh {
                r -= 1
            } else {
                l += 1
            }
        }
        return max_area;
    }
}
// @lc code=end

struct Solution;
fn main() {
    let mut res = Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
    println!("{:?}", res);
    res = Solution::max_area(vec![1, 1]);
    println!("{:?}", res);

    res = Solution::max_area_brute(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
    println!("{:?}", res);
    res = Solution::max_area_brute(vec![1, 1]);
    println!("{:?}", res);
}
