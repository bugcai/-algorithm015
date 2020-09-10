struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn helper(res: &mut Vec<String>, par: String, n: i32, m: i32) {
            if n == 0 && m == 0 {
                res.push(par);
                return;
            }

            if n > 0 { helper(res, par.clone() + "(", n - 1, m + 1); }
            if m > 0 { helper(res, par.clone() + ")", n, m - 1); }
        }
        let mut res = vec![];
        helper(&mut res, String::from(""), n, 0);
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        let result = Solution::generate_parenthesis(3);
        for s in result {
            println!("{}", s);
        }
    }
}

