struct Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut m5, mut m10) = (0, 0);
        for bill in bills {
            match bill {
                5 => m5 += 1,
                10 => {
                    m5 -= 1;
                    m10 += 1;
                }
                20 => {
                    if m10 > 0 {
                        m10 -= 1;
                        m5 -= 1;
                    } else { m5 -= 3; }
                }
                _ => panic!()
            }
            if m5 < 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]));
    }

    #[test]
    fn case2() {
        assert!(Solution::lemonade_change(vec![5, 5, 10]));
    }

    #[test]
    fn case3() {
        assert!(!Solution::lemonade_change(vec![10, 10]));
    }

    #[test]
    fn case4() {
        assert!(!Solution::lemonade_change(vec![5, 5, 10, 10, 20]));
    }

    #[test]
    fn case5() {
        assert!(Solution::lemonade_change(vec![5, 5, 5, 10, 5, 20, 5, 10, 5, 20]));
    }
}