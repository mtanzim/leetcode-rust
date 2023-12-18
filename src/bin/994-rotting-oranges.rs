use std::collections::{HashSet, VecDeque};

struct Solution;

const FRESH: i32 = 1;
const EMPTY: i32 = 0;
const ROTTEN: i32 = 2;

impl Solution {
    fn bfs(grid: &mut Vec<Vec<i32>>) -> i32 {
        let row_len = grid.len() as i32;
        let col_len = grid[0].len() as i32;
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, &value) in row.iter().enumerate() {
                if value == ROTTEN {
                    queue.push_back((row_idx as i32, col_idx as i32))
                }
            }
        }

        let mut day_counter = 0;
        while !queue.is_empty() {
            if let Some((cur_row_idx, cur_col_idx)) = queue.pop_front() {
                let left_n = (cur_row_idx - 1, cur_col_idx);
                let right_n = (cur_row_idx + 1, cur_col_idx);
                let top_n = (cur_row_idx, cur_col_idx - 1);
                let bottom_n = (cur_row_idx, cur_col_idx + 1);
                let all_ns = vec![left_n, top_n, bottom_n, right_n];
                let valid_ns: Vec<(i32, i32)> = all_ns
                    .iter()
                    .filter(|&n| {
                        let (row_idx, col_idx) = n;
                        if *row_idx < 0 || *row_idx >= row_len {
                            return false;
                        }
                        if *col_idx < 0 || *col_idx >= col_len {
                            return false;
                        }
                        return true;
                    })
                    .map(|(r_idx, c_idx)| (*r_idx as i32, *c_idx as i32))
                    .filter(|n| !visited.contains(n))
                    .collect();

                for &n in valid_ns.iter() {
                    let (ri, ci) = n;
                    grid[ri as usize][ci as usize] = ROTTEN;
                    visited.insert(n);
                    queue.push_back(n);
                    day_counter += 1;
                }
            } else {
                break;
            }
        }
        day_counter
    }

    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut cloned_grid = grid.clone();
        let days = Self::bfs(&mut cloned_grid);
        let fresh_left: usize = cloned_grid
            .iter()
            .map(|r| r.iter().filter(|&&orange| orange == FRESH).count())
            .sum();
        if fresh_left > 0 {
            return -1;
        }
        days
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn basic() {
        let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
        assert_eq!(Solution::oranges_rotting(grid), 43)
    }
}
