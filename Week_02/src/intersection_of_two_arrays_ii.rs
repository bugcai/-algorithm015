struct Solution;

impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        nums1.sort();
        nums2.sort();
        let mut i1 = 0;
        let mut i2 = 0;
        while i1 < nums1.len() && i2 < nums2.len() {
            if nums1[i1] == nums2[i2] {
                result.push(nums1[i1]);
                i1 += 1;
                i2 += 1;
            } else if nums1[i1] < nums2[i2] {
                i1 += 1;
            } else {
                i2 += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]), vec![2, 2]);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![4, 9]);
    }
}