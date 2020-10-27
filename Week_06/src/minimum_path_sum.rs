struct Solution;

impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let (row_size, col_size) = (grid.len(), grid[0].len());
        for i in 1..col_size {
            grid[0][i] = grid[0][i - 1] + grid[0][i];
        }
        for i in 1..row_size {
            grid[i][0] = grid[i - 1][0] + grid[i][0];
        }
        for i in 1..row_size {
            for j in 1..col_size {
                grid[i][j] = grid[i][j - 1].min(grid[i - 1][j]) + grid[i][j];
            }
        }
        grid[row_size - 1][col_size - 1]
    }
}

struct Solution2;

impl Solution2 {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let col_size = grid[0].len();
        let mut dp = vec![0; col_size];
        for i in 0..grid.len() {
            for j in 0..col_size {
                match (i, j) {
                    (0, 0) => dp[j] = grid[i][j],
                    (0, _) => dp[j] = dp[j - 1] + grid[i][j],
                    (_, 0) => dp[j] += grid[i][j],
                    (_, _) => dp[j] = dp[j].min(dp[j - 1]) + grid[i][j]
                }
            }
        }
        dp[col_size - 1]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(7, Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]));
        assert_eq!(7, Solution2::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::min_path_sum(vec![vec![1, 2], vec![1, 1]]));
        assert_eq!(3, Solution2::min_path_sum(vec![vec![1, 2], vec![1, 1]]));
    }
}