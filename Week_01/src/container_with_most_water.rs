struct Solution;

/// 双指针
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right, mut max_area) = (0, height.len() - 1, 0);
        while left < right {
            if height[left] < height[right] {
                max_area = max_area.max((right - left) as i32 * height[left]);
                left += 1;
            } else {
                max_area = max_area.max((right - left) as i32 * height[right]);
                right -= 1;
            }
        }
        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}