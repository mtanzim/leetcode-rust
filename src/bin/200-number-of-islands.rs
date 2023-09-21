/*
 * @lc app=leetcode id=200 lang=rust
 *
 * [200] Number of Islands
 */

// @lc code=start

use std::collections::HashSet;
type Coord = (i32, i32);

impl Solution {
    fn dfs(c: Coord, marked: &mut HashSet<Coord>, grid: &Vec<Vec<char>>) {
        if marked.contains(&c) {
            return;
        }
        marked.insert(c);
        // find neighbors
        let (col_idx, row_idx) = c;
        let left_ng = (col_idx - 1, row_idx);
        let right_ng = (col_idx + 1, row_idx);
        let top_ng = (col_idx, row_idx - 1);
        let bot_ng = (col_idx, row_idx + 1);

        let valid_ngs: Vec<Coord> = vec![left_ng, right_ng, top_ng, bot_ng]
            .iter()
            .filter(|(col_ng, row_ng)| {
                if let Some(row) = grid.get(*row_ng as usize) {
                    if let Some(&element) = row.get(*col_ng as usize) {
                        if element == '1' {
                            return true;
                        }
                    }
                }
                return false;
            })
            .map(|(col_ng, row_ng)| (*col_ng, *row_ng))
            .collect();

        for ng in valid_ngs {
            Self::dfs(ng, marked, grid)
        }
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        let mut marked = HashSet::new();
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, _col) in row.iter().enumerate() {
                match row.get(col_idx) {
                    Some(&element)
                        if element == '1'
                            && !marked.contains(&(col_idx as i32, row_idx as i32)) =>
                    {
                        Self::dfs((col_idx as i32, row_idx as i32), &mut marked, &grid);
                        count += 1
                    }
                    _ => {}
                }
            }
        }
        count
    }
}
// @lc code=end

struct Solution;

fn main() {
    let res = Solution::num_islands(vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ]);
    println!("{}", res)
}

mod tests_palindrome {

    #[test]
    fn base_1() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        let res = super::Solution::num_islands(grid);
        let expect = 1;
        assert_eq!(res, expect);
    }
    #[test]
    fn base_2() {
        let grid: Vec<Vec<char>> = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        let res = super::Solution::num_islands(grid);
        let expect = 3;
        assert_eq!(res, expect);
    }
}
