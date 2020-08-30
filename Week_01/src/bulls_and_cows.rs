struct Solution;

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut bulls = 0;
        let mut cows = 0;
        let mut numbers = [0; 10];
        for (s, g) in secret.chars().zip(guess.chars()) {
            let (ns, ng) = (s.to_digit(10).unwrap() as usize, g.to_digit(10).unwrap() as usize);
            if ns == ng {
                bulls += 1;
            } else {
                if numbers[ns] < 0 {
                    cows += 1;
                }
                if numbers[ng] > 0 {
                    cows += 1;
                }
                numbers[ns] += 1;
                numbers[ng] -= 1;
            }
        }
        format!("{}A{}B", bulls, cows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("1A3B", Solution::get_hint(String::from("1807"), String::from("7810")));
    }

    #[test]
    fn case2() {
        assert_eq!("1A1B", Solution::get_hint(String::from("1123"), String::from("0111")));
    }
}