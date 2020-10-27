struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() { return 0; }

        let mut dp = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
        let mut max_side = 0;

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == '1' {
                    dp[i + 1][j + 1] = dp[i][j + 1].min(dp[i][j]).min(dp[i + 1][j]) + 1;
                    max_side = max_side.max(dp[i + 1][j + 1]);
                }
            }
        }
        max_side * max_side
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let matrix = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0']
        ];
        assert_eq!(4, Solution::maximal_square(matrix));
    }
}