/*
 * @lc app=leetcode id=424 lang=rust
 *
 * [424] Longest Repeating Character Replacement
 *
 * https://leetcode.com/problems/longest-repeating-character-replacement/description/
 *
 * algorithms
 * Medium (52.59%)
 * Likes:    9694
 * Dislikes: 423
 * Total Accepted:    582.7K
 * Total Submissions: 1.1M
 * Testcase Example:  '"ABAB"\n2'
 *
 * You are given a string s and an integer k. You can choose any character of
 * the string and change it to any other uppercase English character. You can
 * perform this operation at most k times.
 *
 * Return the length of the longest substring containing the same letter you
 * can get after performing the above operations.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "ABAB", k = 2
 * Output: 4
 * Explanation: Replace the two 'A's with two 'B's or vice versa.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "AABABBA", k = 1
 * Output: 4
 * Explanation: Replace the one 'A' in the middle with 'B' and form "AABBBBA".
 * The substring "BBBB" has the longest repeating letters, which is 4.
 * There may exists other ways to achieve this answer too.
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 10^5
 * s consists of only uppercase English letters.
 * 0 <= k <= s.length
 *
 *
 */


// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut hm: HashMap<char, i32> = HashMap::new();
        let mut l: i32 = 0;
        let mut r: i32 = 0;
        let mut max = 0;
        while r < s.len() as i32 {
            let cur_c = s.chars().nth(r as usize).unwrap();
            let cur_occ = *hm.get(&cur_c).unwrap_or(&0);
            hm.insert(cur_c, cur_occ + 1);
            let updated_occ = *hm.get(&cur_c).unwrap_or(&0);
            let cur_window_len: i32 = r - l + 1;
            let (_, most_freq_c_occ) = hm.iter().fold((cur_c, updated_occ), |acc, (c, i)| {
                let (_, cur_max_i) = acc;
                if *i > cur_max_i {
                    (*c, *i)
                } else {
                    acc
                }
            });

            if cur_window_len - most_freq_c_occ <= k {
                max = max.max(cur_window_len);
            } else {
                let cur_l_c = s.chars().nth(l as usize).unwrap();
                let cur_occ_l = *hm.get(&cur_l_c).unwrap_or(&1);
                hm.insert(cur_l_c, cur_occ_l - 1);
                l += 1;
            }
            r += 1;
        }
        max
    }
}
// @lc code=end
struct Solution;

fn main() {
    println!("{}", Solution::character_replacement("ABAB".to_owned(), 2))
}

mod tests {
    use crate::Solution;

    #[test]
    fn basic() {
        assert_eq!(Solution::character_replacement("ABAB".to_owned(), 2), 4)
    }
    #[test]
    fn basic_2() {
        assert_eq!(Solution::character_replacement("AABABBA".to_owned(), 1), 4)
    }
}
