/*
 * @lc app=leetcode id=875 lang=rust
 *
 * [875] Koko Eating Bananas
 *
 * https://leetcode.com/problems/koko-eating-bananas/description/
 *
 * algorithms
 * Medium (49.97%)
 * Likes:    9396
 * Dislikes: 464
 * Total Accepted:    483K
 * Total Submissions: 973.7K
 * Testcase Example:  '[3,6,7,11]\n8'
 *
 * Koko loves to eat bananas. There are n piles of bananas, the i^th pile has
 * piles[i] bananas. The guards have gone and will come back in h hours.
 *
 * Koko can decide her bananas-per-hour eating speed of k. Each hour, she
 * chooses some pile of bananas and eats k bananas from that pile. If the pile
 * has less than k bananas, she eats all of them instead and will not eat any
 * more bananas during this hour.
 *
 * Koko likes to eat slowly but still wants to finish eating all the bananas
 * before the guards return.
 *
 * Return the minimum integer k such that she can eat all the bananas within h
 * hours.
 *
 *
 * Example 1:
 *
 *
 * Input: piles = [3,6,7,11], h = 8
 * Output: 4
 *
 *
 * Example 2:
 *
 *
 * Input: piles = [30,11,23,4,20], h = 5
 * Output: 30
 *
 *
 * Example 3:
 *
 *
 * Input: piles = [30,11,23,4,20], h = 6
 * Output: 23
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= piles.length <= 10^4
 * piles.length <= h <= 10^9
 * 1 <= piles[i] <= 10^9
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let min_bananas = 1;
        let max_bananas = *piles.iter().max().expect("could not get max");
        let mut l = min_bananas;
        let mut r = max_bananas;
        let mut res = max_bananas;
        while l <= r {
            let cur_bananas_per_h = l + (r - l) / 2;
            let total_h = piles.iter().fold(0.0, |acc, &x| {
                acc + (x as f64 / cur_bananas_per_h as f64).ceil()
            }) as i32;
            if total_h > h {
                l = cur_bananas_per_h + 1
            } else {
                res = res.min(cur_bananas_per_h);
                r = cur_bananas_per_h - 1
            }
        }
        res
    }
}
// @lc code=end

struct Solution;
fn main() {
    let res = Solution::min_eating_speed(vec![3, 6, 7, 11], 8);
    println!("{:?}", res);
}

mod tests {
    use crate::Solution;

    #[test]
    fn basic() {
        assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
    }

    #[test]
    fn basic_1() {
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
    }

    #[test]
    fn basic_2() {
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
    }

    #[test]
    // note: this was fixed by using f64 instead of f32
    fn overflow() {
        assert_eq!(Solution::min_eating_speed(vec![1000000000], 2), 500000000);
    }
}
