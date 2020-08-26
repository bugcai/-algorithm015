struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut i, mut j, mut max_area) = (0, height.len() - 1, 0);
        while i < j {
            let hi = height[i];
            let hj = height[j];
            max_area = max_area.max(hi.min(hj) * (j - i) as i32);
            if hi < hj {
                i += 1;
            } else {
                j -= 1;
            }
        }
        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}