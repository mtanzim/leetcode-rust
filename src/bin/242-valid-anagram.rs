/*
 * @lc app=leetcode id=242 lang=rust
 *
 * [242] Valid Anagram
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut tracker_s = HashMap::new();
        let mut tracker_t = HashMap::new();
        for c in s.chars() {
            let _ = match tracker_s.get(&c) {
                Some(&found) => tracker_s.insert(c, found + 1),
                _ => tracker_s.insert(c, 1),
            };
        }
        for c in t.chars() {
            let _ = match tracker_t.get(&c) {
                Some(&found) => tracker_t.insert(c, found + 1),
                _ => tracker_t.insert(c, 1),
            };
        }

        for (k, v) in tracker_s {
            if !tracker_t.contains_key(&k) {
                return false;
            }
            match tracker_t.get(&k) {
                Some(vt) => {
                    if v != *vt {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }

        true
    }
}
// @lc code=end
struct Solution;
fn main() {
    let res = Solution::is_anagram("hello".to_owned(), "jello".to_owned());
    println!("{}", res)
}

mod tests_valid_anagram {
    #[test]
    fn is_anagram() {
        let res = super::Solution::is_anagram(String::from("anagram"), String::from("naagram"));
        assert_eq!(res, true)
    }
    #[test]
    fn is_not_anagram() {
        let res = super::Solution::is_anagram(String::from("anagramss"), String::from("naagram"));
        assert_eq!(res, false)
    }
}
