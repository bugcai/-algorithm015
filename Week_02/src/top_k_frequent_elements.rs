use std::collections::{HashMap, BinaryHeap};

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freqs = HashMap::new();
        for n in nums {
            let mut nf = freqs.entry(n).or_insert(0);
            *nf += 1;
        }
        let mut min_heap = BinaryHeap::new();
        let mut count = 0;
        for (n, nf) in freqs {
            if count < k {
                min_heap.push((-nf, n));
                count += 1;
            } else if let Some(&(minf, _)) = min_heap.peek() {
                if nf > -minf {
                    min_heap.pop();
                    min_heap.push((-nf, n));
                }
            }
        }
        min_heap.iter().map(|&(_, n)| n).collect()
    }
}

struct Solution2;

impl Solution2 {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq = HashMap::new();
        for &i in &nums {
            let count = freq.entry(i).or_insert(0);
            *count += 1;
        }
        let mut buckets: Vec<Vec<i32>> = vec![vec![]; nums.len() + 1];
        for (i, f) in freq {
            match buckets.get_mut(f as usize) {
                Some(b) => b.push(i),
                _ => buckets.insert(f as usize, vec![i])
            }
        }
        let mut count = 0;
        let mut result = vec![];
        'outer: for b in buckets.iter().rev() {
            for &i in b {
                if count >= k {
                    break 'outer;
                }
                result.push(i);
                count += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn case1() {
        assert_eq!(HashSet::<_>::from_iter(vec![1, 2]), HashSet::<_>::from_iter(Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 5], 2)));
        assert_eq!(HashSet::<_>::from_iter(vec![1, 2]), HashSet::<_>::from_iter(Solution2::top_k_frequent(vec![1, 1, 1, 2, 2, 5], 2)));
    }

    #[test]
    fn case2() {
        assert_eq!(HashSet::<_>::from_iter(vec![1]), HashSet::<_>::from_iter(Solution::top_k_frequent(vec![1], 1)));
        assert_eq!(HashSet::<_>::from_iter(vec![1]), HashSet::<_>::from_iter(Solution2::top_k_frequent(vec![1], 1)));
    }
}