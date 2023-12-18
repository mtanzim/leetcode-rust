struct Solution;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        42
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn basic() {
        let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
        assert_eq!(Solution::oranges_rotting(grid), 44)
    }
}
