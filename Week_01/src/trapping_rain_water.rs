struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut max = 0;
        let mut water = vec![0; height.len()];
        for (i, &h) in height.iter().enumerate() {
            if h > max { max = h; } else { water[i] = max - h; }
        }
        max = 0;
        for &h in height.iter().rev() {
            let mut w = 0;
            if h > max { max = h; } else { w = max - h; }
            ans += w.min(water.pop().unwrap());
        }
        ans
    }
}

struct Solution2;

impl Solution2 {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() == 0 { return 0 as i32; }

        let mut water = 0;
        let (mut left, mut left_max, mut right, mut right_max) = (0, 0, height.len() - 1, 0);
        while left < right {
            left_max = left_max.max(height[left]);
            right_max = right_max.max(height[right]);

            if left_max < right_max {
                water += left_max - height[left];
                left += 1;
            } else {
                water += right_max - height[right];
                right -= 1;
            }
        }
        water
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        assert_eq!(6, Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
        assert_eq!(6, Solution2::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
    }
}