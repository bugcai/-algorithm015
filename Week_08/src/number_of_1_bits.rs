struct Solution;

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let (mut n, mut res) = (n, 0);
        while n > 0 {
            res += 1;
            n &= n - 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::hammingWeight(0b00000000000000000000000000001011));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::hammingWeight(0b00000000000000000000000010000000));
    }

    #[test]
    fn case3() {
        assert_eq!(31, Solution::hammingWeight(0b11111111111111111111111111111101));
    }
}