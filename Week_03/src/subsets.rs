struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![]];

        for n in nums {
            let mut temp = result.clone();
            temp.iter_mut().for_each(|x| x.push(n));
            result.extend(temp);
        }

        result
    }
}

struct Solution2;

impl Solution2 {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(nums: &[i32], path: &mut Vec<i32>, start: usize, result: &mut Vec<Vec<i32>>) {
            result.push(path.clone());
            for i in start..nums.len() {
                path.push(nums[i]);
                backtrack(nums, path, i + 1, result);
                path.pop();
            }
        }
        let mut result = vec![];
        backtrack(&nums[..], &mut vec![], 0, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        assert_eq!(vec![vec![], vec![1], vec![2], vec![1, 2], vec![3], vec![1, 3], vec![2, 3], vec![1, 2, 3]],
                   Solution::subsets(vec![1, 2, 3]));
        assert_eq!(vec![vec![], vec![1], vec![1, 2], vec![1, 2, 3], vec![1, 3], vec![2], vec![2, 3], vec![3]],
                   Solution2::subsets(vec![1, 2, 3]));
    }
}