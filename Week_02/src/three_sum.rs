struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let size = nums.len();
        if size < 3 {
            return result;
        }
        let possible_size = size - 2;
        // sort for accelerating number choice and deduplication
        nums.sort();
        let mut i = 0;
        while i < possible_size {
            let now = nums[i];
            if now > 0 {
                break;
            }
            let target = -now;
            let mut lo = i + 1;
            let mut hi = size - 1;
            while lo < hi {
                let (intLo, intHi) = (nums[lo], nums[hi]);
                let sum = intLo + intHi;
                if sum < target {
                    lo += 1;
                } else if sum > target {
                    hi -= 1;
                } else {
                    result.push(vec![now, nums[lo], nums[hi]]);
                    // deduplicate
                    while lo < hi && nums[lo] == intLo {
                        lo += 1;
                    }
                    while lo < hi && nums[hi] == intHi {
                        hi -= 1;
                    }
                }
            }
            // deduplicate
            while i + 1 < possible_size && now == nums[i + 1] {
                i += 1;
            }
            i += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        assert_eq!(vec![vec![-1, -1, 2], vec![-1, 0, 1]], Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]));
    }
}