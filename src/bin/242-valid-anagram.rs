/*
 * @lc app=leetcode id=242 lang=rust
 *
 * [242] Valid Anagram
 */

struct Solution;
// @lc code=start
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        for (i, c) in s.char_indices() {
            let Some(tc) = t.chars().nth(t.len() - 1 - i) else {
                return  false;
            };
            if tc != c {
                return false;
            }
        }
        return true;
    }
}
// @lc code=end

fn main() {
    let res = Solution::is_anagram("hello".to_owned(), "jello".to_owned());
    println!("{}", res)
}

mod tests {

    #[test]
    fn is_not_anagram() {
        let res = super::Solution::is_anagram("hello".to_owned(), "jello".to_owned());
        assert_eq!(res, false);
    }
    #[test]
    fn is_anagram() {
        let res = super::Solution::is_anagram("hello".to_owned(), "olleh".to_owned());
        assert_eq!(res, true);
    }

    #[test]
    fn anagram_len_mismatch() {
        let res = super::Solution::is_anagram("hello".to_owned(), "ollehss".to_owned());
        assert_eq!(res, false);
    }
    #[test]
    fn empties() {
        let res = super::Solution::is_anagram("hello".to_owned(), "ollehss".to_owned());
        assert_eq!(res, false);
    }
}
