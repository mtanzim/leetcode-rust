/*
 * @lc app=leetcode id=39 lang=rust
 *
 * [39] Combination Sum
 *
 * https://leetcode.com/problems/combination-sum/description/
 *
 * algorithms
 * Medium (70.35%)
 * Likes:    17896
 * Dislikes: 367
 * Total Accepted:    1.7M
 * Total Submissions: 2.4M
 * Testcase Example:  '[2,3,6,7]\n7'
 *
 * Given an array of distinct integers candidates and a target integer target,
 * return a list of all unique combinations of candidates where the chosen
 * numbers sum to target. You may return the combinations in any order.
 *
 * The same number may be chosen from candidates an unlimited number of times.
 * Two combinations are unique if the frequency of at least one of the chosen
 * numbers is different.
 *
 * The test cases are generated such that the number of unique combinations
 * that sum up to target is less than 150 combinations for the given input.
 *
 *
 * Example 1:
 *
 *
 * Input: candidates = [2,3,6,7], target = 7
 * Output: [[2,2,3],[7]]
 * Explanation:
 * 2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple
 * times.
 * 7 is a candidate, and 7 = 7.
 * These are the only two combinations.
 *
 *
 * Example 2:
 *
 *
 * Input: candidates = [2,3,5], target = 8
 * Output: [[2,2,2,2],[2,3,3],[3,5]]
 *
 *
 * Example 3:
 *
 *
 * Input: candidates = [2], target = 1
 * Output: []
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= candidates.length <= 30
 * 2 <= candidates[i] <= 40
 * All elements of candidates are distinct.
 * 1 <= target <= 40
 *
 *
 */


// @lc code=start
use std::{cell::RefCell, rc::Rc};
impl Solution {
    fn dfs(
        i: usize,
        cur: &mut Vec<i32>,
        total: i32,
        target: i32,
        candidates: &Vec<i32>,
        res: Rc<RefCell<Vec<Vec<i32>>>>,
    ) {
        if total == target {
            let mut cur_res = res.borrow_mut();
            cur_res.push(cur.clone());
            return;
        }
        if i >= candidates.len() || total > target {
            return;
        }
        let v = candidates[i];
        cur.push(v);
        Self::dfs(
            i,
            cur,
            total + v,
            target,
            candidates,
            res.clone(),
        );
        cur.pop();
        Self::dfs(i + 1, cur, total, target, candidates, res.clone())
    }
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let res: Rc<RefCell<Vec<Vec<i32>>>> = Rc::new(RefCell::new(vec![]));
        Self::dfs(0, &mut vec![], 0, target, &candidates, res.clone());
        let results = res.borrow();
        results.to_vec()
    }
}
// @lc code=end

struct Solution;

fn main() {
    println!("hello");
    let c = vec![2, 3, 6, 7];
    let t = 7;
    let ans = Solution::combination_sum(c, t);
    println!("{:?}", ans)
}

mod tests {
    use crate::Solution;


    #[test]
    fn basic() {
        let c = vec![2, 3, 6, 7];
        let t = 7;
        let ans = Solution::combination_sum(c, t);

        assert_eq!(ans, vec![vec![2, 2, 3], vec![7]])
    }
}
