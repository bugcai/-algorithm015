struct Solution;

impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut res = 0_u32;
        for _ in 0..32 {
            res = (res << 1) + (x & 1);
            x >>= 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0b00111001011110000010100101000000, Solution::reverse_bits(0b00000010100101000001111010011100));
    }

    #[test]
    fn case2() {
        assert_eq!(0b10111111111111111111111111111111, Solution::reverse_bits(0b11111111111111111111111111111101));
    }
}