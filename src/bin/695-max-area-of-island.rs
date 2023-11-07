/*
 * @lc app=leetcode id=695 lang=rust
 *
 * [695] Max Area of Island
 *
 * https://leetcode.com/problems/max-area-of-island/description/
 *
 * algorithms
 * Medium (71.87%)
 * Likes:    9598
 * Dislikes: 198
 * Total Accepted:    777.8K
 * Total Submissions: 1.1M
 * Testcase Example:  '[[0,0,1,0,0,0,0,1,0,0,0,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,1,1,0,1,0,0,0,0,0,0,0,0],[0,1,0,0,1,1,0,0,1,0,1,0,0],[0,1,0,0,1,1,0,0,1,1,1,0,0],[0,0,0,0,0,0,0,0,0,0,1,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,0,0,0,0,0,0,1,1,0,0,0,0]]'
 *
 * You are given an m x n binary matrix grid. An island is a group of 1's
 * (representing land) connected 4-directionally (horizontal or vertical.) You
 * may assume all four edges of the grid are surrounded by water.
 *
 * The area of an island is the number of cells with a value 1 in the island.
 *
 * Return the maximum area of an island in grid. If there is no island, return
 * 0.
 *
 *
 * Example 1:
 *
 *
 * Input: grid =
 * [[0,0,1,0,0,0,0,1,0,0,0,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,1,1,0,1,0,0,0,0,0,0,0,0],[0,1,0,0,1,1,0,0,1,0,1,0,0],[0,1,0,0,1,1,0,0,1,1,1,0,0],[0,0,0,0,0,0,0,0,0,0,1,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,0,0,0,0,0,0,1,1,0,0,0,0]]
 * Output: 6
 * Explanation: The answer is not 11, because the island must be connected
 * 4-directionally.
 *
 *
 * Example 2:
 *
 *
 * Input: grid = [[0,0,0,0,0,0,0,0]]
 * Output: 0
 *
 *
 *
 * Constraints:
 *
 *
 * m == grid.length
 * n == grid[i].length
 * 1 <= m, n <= 50
 * grid[i][j] is either 0 or 1.
 *
 *
 */

// @lc code=start
use std::collections::HashSet;
type Coord = (i32, i32);
impl Solution {
    fn dfs(c: Coord, marked: &mut HashSet<Coord>, grid: &Vec<Vec<i32>>) -> i32 {
        if marked.contains(&c) {
            return 0;
        }
        marked.insert(c);
        let (row, col) = c;
        let v = grid[row as usize][col as usize];
        let all_neighbors = vec![
            (row + 1, col),
            (row - 1, col),
            (row, col + 1),
            (row, col - 1),
        ];
        let nv: i32 = all_neighbors
            .iter()
            .filter(|(row, col)| {
                *row >= 0 && *col >= 0 && *row < grid.len() as i32 && *col < grid[0].len() as i32
            })
            .filter(|(row, col)| grid[*row as usize][*col as usize] == 1)
            .map(|(row, col)| Self::dfs((*row, *col), marked, grid))
            .sum();
        nv + v
    }
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut max_area = 0;
        let mut marked = HashSet::new();
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, v) in row.iter().enumerate() {
                if *v == 1 {
                    let cur_area = Self::dfs((row_idx as i32, col_idx as i32), &mut marked, &grid);
                    max_area = max_area.max(cur_area)
                }
            }
        }
        max_area
    }
}
// @lc code=end
struct Solution;
fn main() {
    let input = vec![vec![1, 2], vec![2, 3]];
    println!("{:?}", Solution::max_area_of_island(input));
}

mod tests_max_area_of_island {
    use crate::Solution;

    #[test]
    fn basic() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];
        assert_eq!(6, Solution::max_area_of_island(matrix))
    }

    #[test]
    fn failing() {
        let matrix: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(1, Solution::max_area_of_island(matrix))
    }
}
