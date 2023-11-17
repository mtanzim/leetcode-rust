/*
* @lc app=leetcode id=295 lang=rust
*
* [295] Find Median from Data Stream
*
* https://leetcode.com/problems/find-median-from-data-stream/description/
*
* algorithms
* Hard (51.51%)
* Likes:    11330
* Dislikes: 223
* Total Accepted:    702.6K
* Total Submissions: 1.4M
* Testcase Example:  '["MedianFinder","addNum","addNum","findMedian","addNum","findMedian"]\n' +
 '[[],[1],[2],[],[3],[]]'
*
* The median is the middle value in an ordered integer list. If the size of
* the list is even, there is no middle value, and the median is the mean of
* the two middle values.
*
*
* For example, for arr = [2,3,4], the median is 3.
* For example, for arr = [2,3], the median is (2 + 3) / 2 = 2.5.
*
*
* Implement the MedianFinder class:
*
*
* MedianFinder() initializes the MedianFinder object.
* void addNum(int num) adds the integer num from the data stream to the data
* structure.
* double findMedian() returns the median of all elements so far. Answers
* within 10^-5 of the actual answer will be accepted.
*
*
*
* Example 1:
*
*
* Input
* ["MedianFinder", "addNum", "addNum", "findMedian", "addNum", "findMedian"]
* [[], [1], [2], [], [3], []]
* Output
* [null, null, null, 1.5, null, 2.0]
*
* Explanation
* MedianFinder medianFinder = new MedianFinder();
* medianFinder.addNum(1);    // arr = [1]
* medianFinder.addNum(2);    // arr = [1, 2]
* medianFinder.findMedian(); // return 1.5 (i.e., (1 + 2) / 2)
* medianFinder.addNum(3);    // arr[1, 2, 3]
* medianFinder.findMedian(); // return 2.0
*
*
*
* Constraints:
*
*
* -10^5 <= num <= 10^5
* There will be at least one element in the data structure before calling
* findMedian.
* At most 5 * 10^4 calls will be made to addNum and findMedian.
*
*
*
* Follow up:
*
*
* If all integer numbers from the stream are in the range [0, 100], how would
* you optimize your solution?
* If 99% of all integer numbers from the stream are in the range [0, 100], how
* would you optimize your solution?
*
*
*/
#[macro_use]
extern crate approx;
// @lc code=start
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
// TODO: clean up unwraps and expects!
impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            left_heap: BinaryHeap::new(),
            right_heap: BinaryHeap::new(),
        }
    }

    // TODO: clean up boolean conditions here
    fn add_num(&mut self, num: i32) {
        if self.left_heap.len() > 0 {
            if *self.left_heap.peek().unwrap() >= num {
                self.left_heap.push(num);
            } else {
                self.right_heap.push(Reverse(num));
            }
        } else {
            self.left_heap.push(num);
        }
        Self::fix_heaps(self);
    }

    fn fix_heaps(&mut self) {
        if self.left_heap.len() as i32 - self.right_heap.len() as i32 > 1 {
            let temp = self.left_heap.pop().expect("invalid op on left heap");
            self.right_heap.push(Reverse(temp))
        } else if self.right_heap.len() as i32 - self.left_heap.len() as i32 > 1 {
            let Reverse(temp) = self.right_heap.pop().expect("invalid op on right heap");
            self.left_heap.push(temp)
        }
    }

    fn find_median(&self) -> f64 {
        let left_len = self.left_heap.len();
        let right_len = self.right_heap.len();

        if left_len == right_len {
            let lv = self.left_heap.peek().unwrap();
            let Reverse(rv) = self.right_heap.peek().unwrap();
            return (*lv + *rv) as f64 / 2.0;
        }
        if left_len > right_len {
            let lv = self.left_heap.peek().unwrap();
            return *lv as f64;
        }
        let Reverse(rv) = self.right_heap.peek().unwrap();
        return *rv as f64;
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
struct MedianFinder {
    left_heap: BinaryHeap<i32>,
    right_heap: BinaryHeap<Reverse<i32>>,
}

// @lc code=end

fn main() {}

mod tests {

    use crate::MedianFinder;
    #[test]
    fn basic() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        let r1 = mf.find_median();
        assert!(relative_eq!(r1, 1.5, epsilon = f64::EPSILON));

        mf.add_num(3);
        let r2 = mf.find_median();
        assert!(relative_eq!(r2, 2.0, epsilon = f64::EPSILON));
    }
}
