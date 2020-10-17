struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        fn dfs(board: &mut Vec<Vec<char>>, empties: &mut Vec<(usize, usize)>, rows: &mut Vec<u32>, columns: &mut Vec<u32>, boxes: &mut Vec<u32>) -> bool {
            if let Some((i, j)) = empties.pop() {
                let box_index = (i / 3) * 3 + j / 3;
                let bits: u32 = !(rows[i] | columns[j] | boxes[box_index]);
                for d in 0..9_u8 {
                    let mask = 1 << d;
                    if bits & mask != 0 {
                        board[i][j] = (d + b'1') as char;
                        rows[i] |= mask;
                        columns[j] |= mask;
                        boxes[box_index] |= mask;
                        if dfs(board, empties, rows, columns, boxes) {
                            return true;
                        }
                        board[i][j] = '.';
                        rows[i] ^= mask;
                        columns[j] ^= mask;
                        boxes[box_index] ^= mask;
                    }
                }
                empties.push((i, j));
                return false;
            }
            true
        }

        let (mut rows, mut columns, mut boxes) = (vec![0_u32; 9], vec![0_u32; 9], vec![0_u32; 9]);
        let mut empties = vec![];
        for i in 0..9 {
            for j in 0..9 {
                let value = board[i][j];
                if value == '.' {
                    empties.push((i, j));
                } else {
                    let mask = 1 << (value as u8 - b'1');
                    let box_index = (i / 3) * 3 + j / 3;
                    rows[i] |= mask;
                    columns[j] |= mask;
                    boxes[box_index] |= mask;
                }
            }
        }

        dfs(board, &mut empties, &mut rows, &mut columns, &mut boxes);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        Solution::solve_sudoku(&mut board);

        println!("{:?}", board);
    }
}