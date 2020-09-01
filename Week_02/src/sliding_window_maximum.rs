use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut result = Vec::with_capacity(nums.len() - k + 1);
        let mut deque = VecDeque::with_capacity(k);
        for i in 0..nums.len() {
            //如果队列中队头元素和当前元素位置相差i-k，相当于队头元素要
            //出窗口了，就把队头元素给移除，注意队列中存储的是元素的下标
            if let Some(&front) = deque.front() {
                if front + k <= i {
                    deque.pop_front();
                }
            }
            //在添加一个值之前，前面比他小的都要被移除掉，并且还要保证
            //窗口中队列头部元素永远是队列中最大的
            while !deque.is_empty() && nums[*deque.back().unwrap()] < nums[i] {
                deque.pop_back();
            }

            //当前元素的下标加入到队列的尾部
            deque.push_back(i);
            //当窗口的长度大于等于k个的时候才开始计算（注意这里的i是从0开始的）
            if i >= k - 1 {
                //队头元素是队列中最大的，把队列头部的元素加入到数组中
                result.push(nums[*deque.front().unwrap()]);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        assert_eq!(vec![3, 3, 5, 5, 6, 7], Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3));
    }
}