struct Solution;

impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        let mut s: Vec<i32> = vec![-1];
        heights.push(0);
        let mut max_area = 0;
        for i in 0..heights.len() {
            let mut top: i32 = *s.last().unwrap();
            while top != -1 && heights[i] < heights[top as usize] {
                let h = heights[s.pop().unwrap() as usize];
                top = *s.last().unwrap();
                max_area = max_area.max(h * (i as i32 - top - 1));
            }
            s.push(i as i32);
        }
        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }
}