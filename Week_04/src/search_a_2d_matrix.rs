struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() == 0 || matrix[0].len() == 0 { return false; }
        let (row_len, col_len) = (matrix.len(), matrix[0].len());
        let (mut lo, mut hi) = (0, row_len * col_len - 1);
        while lo <= hi {
            let mid = (lo + hi) / 2;
            let mid_num = matrix[mid / col_len][mid % col_len];
            if mid_num == target { return true; }
            if mid_num < target {
                lo = mid + 1;
            } else {
                if mid == 0 { return false; }
                hi = mid - 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {

    }
}