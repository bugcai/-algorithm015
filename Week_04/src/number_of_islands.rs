use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        if grid.len() == 0 || grid[0].len() == 0 { return 0; }
        let (h, w) = (grid.len() - 1, grid[0].len() - 1);
        let mut q = VecDeque::new();

        let mut ans = 0;
        for ir in 0..=h {
            for ic in 0..=w {
                if grid[ir][ic] == '0' { continue; }
                grid[ir][ic] = '0';
                q.push_back((ir, ic));
                while let Some((r, c)) = q.pop_front() {
                    if r < h && grid[r + 1][c] == '1' {
                        grid[r + 1][c] = '0';
                        q.push_back((r + 1, c));
                    }
                    if r > 0 && grid[r - 1][c] == '1' {
                        grid[r - 1][c] = '0';
                        q.push_back((r - 1, c));
                    }
                    if c < w && grid[r][c + 1] == '1' {
                        grid[r][c + 1] = '0';
                        q.push_back((r, c + 1));
                    }
                    if c > 0 && grid[r][c - 1] == '1' {
                        grid[r][c - 1] = '0';
                        q.push_back((r, c - 1));
                    }
                }
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0']
        ];
        assert_eq!(1, Solution::num_islands(grid));
    }

    #[test]
    fn case2() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1']
        ];
        assert_eq!(3, Solution::num_islands(grid));
    }
}