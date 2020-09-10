struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        fn render(board: &Vec<usize>, size: usize) -> Vec<String> {
            board.iter().map(|&i| { ".".repeat(i) + "Q" + &".".repeat(size - i - 1) }).collect()
        }
        fn check(board: &[usize], queen_pos: (usize, usize)) -> bool {
            for i in 0..queen_pos.0 {
                if queen_pos.1 == board[i] || (queen_pos.0 as i32 - i as i32).abs() == (queen_pos.1 as i32 - board[i] as i32).abs() {
                    return false;
                }
            }
            true
        }
        fn back_track(size: usize, row: usize, board: &mut Vec<usize>, res: &mut Vec<Vec<String>>) {
            if row >= size {
                res.push(render(board, size));
                return;
            }
            for col in 0..size {
                if check(&board[..], (row, col)) {
                    board[row] = col;
                    back_track(size, row + 1, board, res);
                }
            }
        }
        let mut board = vec![0; n as usize];
        let mut res = vec![];
        back_track(n as usize, 0, &mut board, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        println!("{:?}", Solution::solve_n_queens(4));
    }
}