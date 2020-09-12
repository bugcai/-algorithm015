struct Solution;

// 快速幂 + 迭代
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let (mut x, mut n) = if n < 0 { (1f64 / x, -n) } else { (x, n) };
        let mut pow = 1f64;

        if n == 0 { return 1f64; }

        while n != 1 {
            if n % 2 == 1 {
                pow = pow * x;
            }
            n = n / 2;
            x = x * x;
            if x == 0f64 || x == 1.0f64 { break; }
        }
        pow * x
    }
}

struct Solution2;

// 快速幂 + 递归
impl Solution2 {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        fn helper(x: f64, n: i32) -> f64 {
            if n == 0 {
                return 1f64;
            }
            if x == 0f64 || x == 1.0f64 { return x; };
            let y = helper(x * x, n / 2);
            if n % 2 == 0 { y } else { x * y }
        }

        let (x, n) = if n < 0 { (1f64 / x, -n) } else { (x, n) };
        helper(x, n)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::my_pow(2.00000, 10), 1024.00000);
        assert_eq!(Solution2::my_pow(2.00000, 10), 1024.00000);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::my_pow(2.10000, 3), 9.261000000000001);
        assert_eq!(Solution2::my_pow(2.00000, 10), 1024.00000);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::my_pow(2.00000, -2), 0.25000);
        assert_eq!(Solution2::my_pow(2.00000, 10), 1024.00000);
    }
}

