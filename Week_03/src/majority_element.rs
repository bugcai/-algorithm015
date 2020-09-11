struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut count = 0;
        for n in nums {
            if count == 0 {
                i = n;
            }
            count += if n == i { 1 } else { -1 };
        }
        i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::majority_element(vec![3, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
    }
}