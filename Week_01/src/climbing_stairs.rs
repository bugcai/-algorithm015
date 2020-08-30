struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }
        let mut first = 1;
        let mut second = 2;
        let mut third = 0;
        for _i in 3..n + 1 {
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
    fn case1() {
        assert_eq!(2, Solution::climb_stairs(2));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::climb_stairs(3));
    }

    #[test]
    fn case3() {
        assert_eq!(5, Solution::climb_stairs(4));
    }
}