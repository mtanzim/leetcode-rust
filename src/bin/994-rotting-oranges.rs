use std::collections::VecDeque;

struct Solution;

const FRESH: i32 = 1;
const EMPTY: i32 = 0;
const ROTTEN: i32 = 2;

impl Solution {
    fn bfs(grid: &mut Vec<Vec<i32>>) -> i32 {
        let row_len = grid.len() as i32;
        let col_len = grid[0].len() as i32;
        let mut fresh_left: usize = grid
            .iter()
            .map(|r| r.iter().filter(|&&orange| orange == FRESH).count())
            .sum();
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, &value) in row.iter().enumerate() {
                if value == ROTTEN {
                    queue.push_back((row_idx as i32, col_idx as i32))
                }
            }
        }

        let mut day_counter = 0;
        loop {
            if fresh_left <= 0 {
                break;
            }
            if queue.is_empty() {
                break;
            }
            let queue_cur_len = queue.len();
            for _ in 0..queue_cur_len {
                let (cur_row_idx, cur_col_idx) =
                    queue.pop_front().expect("invalid pop on queue");
                let left_n = (cur_row_idx - 1, cur_col_idx);
                let right_n = (cur_row_idx + 1, cur_col_idx);
                let top_n = (cur_row_idx, cur_col_idx - 1);
                let bottom_n = (cur_row_idx, cur_col_idx + 1);
                let all_ns = vec![left_n, top_n, bottom_n, right_n];
                let valid_ns: Vec<(i32, i32)> = all_ns
                    .iter()
                    .filter(|&n| {
                        let (row_idx, col_idx) = *n;
                        if row_idx < 0 || row_idx >= row_len {
                            return false;
                        }
                        if col_idx < 0 || col_idx >= col_len {
                            return false;
                        }
                        if grid[row_idx as usize][col_idx as usize] != FRESH {
                            return false;
                        }
                        return true;
                    })
                    .map(|(r_idx, c_idx)| (*r_idx as i32, *c_idx as i32))
                    .collect();

                for &n in valid_ns.iter() {
                    let (ri, ci) = n;
                    grid[ri as usize][ci as usize] = ROTTEN;
                    fresh_left -= 1;
                    queue.push_back(n);
                }
            }
            day_counter += 1
        }
        if fresh_left > 0 {
            return -1;
        }
        day_counter
    }

    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut cloned_grid = grid.clone();
        Self::bfs(&mut cloned_grid) as i32
    }
}

fn main() {
    let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
    println!("{:?}: {}", grid.clone(), Solution::oranges_rotting(grid))
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn basic() {
        let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
        assert_eq!(Solution::oranges_rotting(grid), 4)
    }
}
