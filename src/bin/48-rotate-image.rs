/*
 * @lc app=leetcode id=48 lang=rust
 *
 * [48] Rotate Image
 *
 * https://leetcode.com/problems/rotate-image/description/
 *
 * algorithms
 * Medium (72.97%)
 * Likes:    16503
 * Dislikes: 726
 * Total Accepted:    1.5M
 * Total Submissions: 2M
 * Testcase Example:  '[[1,2,3],[4,5,6],[7,8,9]]'
 *
 * You are given an n x n 2D matrix representing an image, rotate the image by
 * 90 degrees (clockwise).
 *
 * You have to rotate the image in-place, which means you have to modify the
 * input 2D matrix directly. DO NOT allocate another 2D matrix and do the
 * rotation.
 *
 *
 * Example 1:
 *
 *
 * Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
 * Output: [[7,4,1],[8,5,2],[9,6,3]]
 *
 *
 * Example 2:
 *
 *
 * Input: matrix = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
 * Output: [[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]
 *
 *
 *
 * Constraints:
 *
 *
 * n == matrix.length == matrix[i].length
 * 1 <= n <= 20
 * -1000 <= matrix[i][j] <= 1000
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        for (row_idx, row) in matrix.clone().iter().enumerate() {
            for (col_idx, _) in row.clone().iter().enumerate() {
                if row_idx < col_idx {
                    continue;
                }
                let temp = matrix[row_idx][col_idx];
                matrix[row_idx][col_idx] = matrix[col_idx][row_idx];
                matrix[col_idx][row_idx] = temp;
            }
        }
        for (row_idx, _) in matrix.clone().iter().enumerate() {
            matrix[row_idx].reverse()
        }
    }
}
// @lc code=end

struct Solution;

mod tests {
    use crate::Solution;

    #[test]
    fn basic() {
        let mut matrix: Vec<Vec<i32>> = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        let expected: Vec<Vec<i32>> = vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, expected);
    }
}
