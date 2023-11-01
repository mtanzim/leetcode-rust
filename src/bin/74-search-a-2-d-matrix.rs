/*
 * @lc app=leetcode id=74 lang=rust
 *
 * [74] Search a 2D Matrix
 *
 * https://leetcode.com/problems/search-a-2d-matrix/description/
 *
 * algorithms
 * Medium (49.18%)
 * Likes:    14740
 * Dislikes: 384
 * Total Accepted:    1.5M
 * Total Submissions: 3.1M
 * Testcase Example:  '[[1,3,5,7],[10,11,16,20],[23,30,34,60]]\n3'
 *
 * You are given an m x n integer matrix matrix with the following two
 * properties:
 *
 *
 * Each row is sorted in non-decreasing order.
 * The first integer of each row is greater than the last integer of the
 * previous row.
 *
 *
 * Given an integer target, return true if target is in matrix or false
 * otherwise.
 *
 * You must write a solution in O(log(m * n)) time complexity.
 *
 *
 * Example 1:
 *
 *
 * Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
 * Output: false
 *
 *
 *
 * Constraints:
 *
 *
 * m == matrix.length
 * n == matrix[i].length
 * 1 <= m, n <= 100
 * -10^4 <= matrix[i][j], target <= 10^4
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mins: Vec<i32> = matrix
            .iter()
            .map(|a| *a.first().expect("empty array, panic"))
            .collect();
        let maxes: Vec<i32> = matrix
            .iter()
            .map(|a| *a.last().expect("empty array, panic"))
            .collect();
        let row_idx: i32 = (|| {
            let mut l: i32 = 0;
            let mut r: i32 = mins.len() as i32 - 1;
            while l <= r {
                let mid = (l + r) / 2;
                let mid_min = mins[mid as usize];
                let mid_max = maxes[mid as usize];
                if target >= mid_min && target <= mid_max {
                    return mid as i32;
                }
                if target > mid_max {
                    l = mid + 1
                } else {
                    r = mid - 1
                }
            }
            return -1;
        })();
        if row_idx < 0 {
            return false;
        }
        let row = &matrix[row_idx as usize];
        let mut l = 0;
        let mut r = row.len() as i32 - 1;
        while l <= r {
            let mid = (l + r) / 2;
            let val = row[mid as usize];
            if val == target {
                return true;
            }
            if target > val {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        false
    }
}
// @lc code=end

struct Solution;

mod tests {
    use crate::Solution;

    #[test]
    fn basic() {
        let matrix: Vec<Vec<i32>> =
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert_eq!(Solution::search_matrix(matrix.clone(), 3), true);
        assert_eq!(Solution::search_matrix(matrix.clone(), 11), true);
        assert_eq!(Solution::search_matrix(matrix.clone(), 30), true);
        assert_eq!(Solution::search_matrix(matrix.clone(), 1), true);
        assert_eq!(Solution::search_matrix(matrix.clone(), 7), true);
        assert_eq!(Solution::search_matrix(matrix.clone(), 60), true);
    }
}
