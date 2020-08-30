struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if n == 0 {
            return;
        }

        nums1.rotate_right(n as usize);

        let mut i1 = n as usize;
        let mut i2 = 0usize;
        let mut i = 0usize;
        while i1 < (m + n) as usize && i2 < n as usize {
            if nums1[i1] < nums2[i2] {
                nums1[i] = nums1[i1];
                i1 += 1;
            } else {
                nums1[i] = nums2[i2];
                i2 += 1;
            }
            i += 1;
            println!("{:?}", nums1);
        }
        while i2 < n as usize {
            nums1[i] = nums2[i2];
            i += 1;
            i2 += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        Solution::merge(&mut nums1, 3, &mut vec![2, 5, 6], 3);
        assert_eq!(vec![1, 2, 2, 3, 5, 6], nums1);
    }

    #[test]
    fn case2() {
        let mut nums1 = vec![1];
        Solution::merge(&mut nums1, 1, &mut vec![], 0);
        assert_eq!(vec![1], nums1);
    }

    #[test]
    fn case3() {
        let mut nums1 = vec![4, 0, 0, 0, 0, 0];
        Solution::merge(&mut nums1, 1, &mut vec![1, 2, 3, 5, 6], 5);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], nums1);
    }
}