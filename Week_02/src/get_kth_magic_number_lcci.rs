struct Solution;

impl Solution {
    pub fn get_kth_magic_number(k: i32) -> i32 {
        let mut ans = vec![1];
        let (mut a, mut b, mut c) = (0, 0, 0);
        for i in 1..k as usize {
            ans.push(*[ans[a] * 3, ans[b] * 5, ans[c] * 7].iter().min().unwrap());
            a += (ans[i] == ans[a] * 3) as usize;
            b += (ans[i] == ans[b] * 5) as usize;
            c += (ans[i] == ans[c] * 7) as usize;
        }
        ans[k as usize - 1] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        assert_eq!(9, Solution::get_kth_magic_number(5));
    }
}