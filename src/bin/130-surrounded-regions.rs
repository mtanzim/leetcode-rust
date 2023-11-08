/*
 * @lc app=leetcode id=130 lang=rust
 *
 * [130] Surrounded Regions
 *
 * https://leetcode.com/problems/surrounded-regions/description/
 *
 * algorithms
 * Medium (37.93%)
 * Likes:    8067
 * Dislikes: 1673
 * Total Accepted:    613.9K
 * Total Submissions: 1.6M
 * Testcase Example:  '[["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]'
 *
 * Given an m x n matrix board containing 'X' and 'O', capture all regions that
 * are 4-directionally surrounded by 'X'.
 * 
 * A region is captured by flipping all 'O's into 'X's in that surrounded
 * region.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: board =
 * [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]
 * Output:
 * [["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X","X"]]
 * Explanation: Notice that an 'O' should not be flipped if:
 * - It is on the border, or
 * - It is adjacent to an 'O' that should not be flipped.
 * The bottom 'O' is on the border, so it is not flipped.
 * The other three 'O' form a surrounded region, so they are flipped.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: board = [["X"]]
 * Output: [["X"]]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * m == board.length
 * n == board[i].length
 * 1 <= m, n <= 200
 * board[i][j] is 'X' or 'O'.
 * 
 * 
 */

// @lc code=start
/**
 Do not return anything, modify board in-place instead.
 */
impl Solution {
  pub fn solve(board: &mut Vec<Vec<char>>) {
  }
}
// @lc code=end
struct Solution;

fn main() {
  let mut  matrix = vec![vec!['1','2']];
  println!("{:?}", Solution::solve(&mut matrix));
}
