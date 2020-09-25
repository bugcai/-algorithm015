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
        for (j, &h) in height.iter().rev().enumerate() {
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

        let mut left = 0;
        let mut left_max = 0;
        let mut right_max = 0;
        let mut right = height.len() - 1;
        let mut water = 0;

        while left != right {
            left_max = height[left].max(left_max);
            right_max = height[right].max(right_max);

            if left_max < right_max {
                water += left_max - height[left];
                left += 1;
            } else {
                water += right_max - height[right];
                right -= 1;
            }
        }
        water as i32
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