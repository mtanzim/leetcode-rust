/*
 * @lc app=leetcode id=125 lang=rust
 *
 * [125] Valid Palindrome
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let cleaned: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
        let lc = cleaned.to_lowercase();
        let reversed: String = lc.chars().rev().collect();
        for (i, c) in lc.char_indices() {
            match reversed.chars().nth(i) {
                Some(tc) => {
                    if tc != c {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        return true;
    }
}
// @lc code=end
// @lc code=end
struct Solution;

fn main() {
    let res = Solution::is_palindrome("hello".to_owned());
    println!("{}", res)
}

mod tests_palindrome {

    #[test]
    fn is_not_palindrome() {
        let res = super::Solution::is_palindrome("hello".to_owned());
        assert_eq!(res, false);
    }
    #[test]
    fn is_palindrome() {
        let res = super::Solution::is_palindrome("tacocat".to_owned());
        assert_eq!(res, true);
    }

    #[test]
    fn empties() {
        let res = super::Solution::is_palindrome("".to_owned());
        assert_eq!(res, true);
    }
}
