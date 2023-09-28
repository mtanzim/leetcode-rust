/*
 * @lc app=leetcode id=121 lang=rust
 *
 * [121] Best Time to Buy and Sell Stock
 *
 * https://leetcode.com/problems/best-time-to-buy-and-sell-stock/description/
 *
 * algorithms
 * Easy (53.52%)
 * Likes:    28521
 * Dislikes: 968
 * Total Accepted:    3.8M
 * Total Submissions: 7.1M
 * Testcase Example:  '[7,1,5,3,6,4]'
 *
 * You are given an array prices where prices[i] is the price of a given stock
 * on the i^th day.
 *
 * You want to maximize your profit by choosing a single day to buy one stock
 * and choosing a different day in the future to sell that stock.
 *
 * Return the maximum profit you can achieve from this transaction. If you
 * cannot achieve any profit, return 0.
 *
 *
 * Example 1:
 *
 *
 * Input: prices = [7,1,5,3,6,4]
 * Output: 5
 * Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit
 * = 6-1 = 5.
 * Note that buying on day 2 and selling on day 1 is not allowed because you
 * must buy before you sell.
 *
 *
 * Example 2:
 *
 *
 * Input: prices = [7,6,4,3,1]
 * Output: 0
 * Explanation: In this case, no transactions are done and the max profit =
 * 0.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= prices.length <= 10^5
 * 0 <= prices[i] <= 10^4
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 1 {
            return 0;
        }
        let mut min_price = prices[0];
        let mut cur_max = 0;
        for p in prices[1..].into_iter() {
            let cur_profit = p - min_price;
            if *p < min_price {
                min_price = *p;
            }
            if cur_profit > cur_max {
                cur_max = cur_profit;
            }
        }
        cur_max
    }
}
// @lc code=end

struct Solution;
fn main() {
    let res = Solution::max_profit(vec![7, 1, 5, 3, 6, 4]);
    println!("{:?}", res)
}

mod tests_best_time_to_buy_stock {

    #[test]
    fn base() {
        let res = super::Solution::max_profit(vec![7, 1, 5, 3, 6, 4]);
        let expect = 5;
        assert_eq!(res, expect)
    }
}
