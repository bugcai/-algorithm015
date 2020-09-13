struct Solution;

impl Solution {
    pub fn get_kth_magic_number(k: i32) -> i32 {
        let k = k as usize;
        let mut nums = vec![1];
        let (mut a, mut b, mut c) = (0, 0, 0);
        for _ in 1..k {
            let (n3, n5, n7) = (nums[a] * 3, nums[b] * 5, nums[c] * 7);
            let magic_number = *[n3, n5, n7].iter().min().unwrap();
            nums.push(magic_number);
            a += (magic_number == n3) as usize;
            b += (magic_number == n5) as usize;
            c += (magic_number == n7) as usize;
        }
        nums[k - 1]
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