struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut ugly = vec![1];
        let (mut p2, mut p3, mut p5) = (0, 0, 0);
        let mut min = 1;
        for _ in 2..=n {
            let (u2, u3, u5) = (ugly[p2] * 2, ugly[p3] * 3, ugly[p5] * 5);
            min = u2.min(u3).min(u5);
            ugly.push(min);
            if u2 == min {
                p2 += 1;
            }
            if u3 == min {
                p3 += 1;
            }
            if u5 == min {
                p5 += 1;
            }
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        assert_eq!(12, Solution::nth_ugly_number(10));
    }
}