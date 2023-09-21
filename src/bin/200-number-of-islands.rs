/*
 * @lc app=leetcode id=200 lang=rust
 *
 * [200] Number of Islands
 */

// @lc code=start
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        3
    }
}
// @lc code=end
struct Solution;

fn main() {
    let res = Solution::num_islands(vec![vec!['1', '0'], vec!['1', '0']]);
    println!("{}", res)
}

mod tests_palindrome {

    #[test]
    fn is_not_palindrome() {
        let res = super::Solution::num_islands(vec![vec!['1', '0'], vec!['1', '0']]);
        assert_eq!(res, 3);
    }
}
