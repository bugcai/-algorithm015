use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let map = arr2.iter().zip(0..).collect::<HashMap<_, _>>();
        arr1.sort_unstable_by_key(|x| *map.get(x).unwrap_or(&(x + 1000)));
        arr1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let ans = Solution::relative_sort_array(vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19], vec![2, 1, 4, 3, 9, 6]);
        let expect = vec![2,2,2,1,4,3,3,9,6,7,19];
        assert_eq!(expect, ans);
    }
}