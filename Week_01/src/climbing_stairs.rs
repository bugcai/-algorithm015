struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        let mut first = 1;
        let mut second = 2;
        let mut third = 3;
        for _i in 4..n + 1 {
            third = first + second;
            first = second;
            second = third;
        }
        return third;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular() {
        assert_eq!(3, Solution::climb_stairs(3));
    }
}