struct DisjointSet {
    parents: Vec<usize>,
    count: usize,
}

impl DisjointSet {
    fn with_capacity(n: usize) -> Self {
        let mut v = vec![0; n];
        for i in 0..n {
            v[i] = i;
        }
        DisjointSet {
            parents: v,
            count: n,
        }
    }

    pub fn find(&mut self, mut p: usize) -> usize {
        while self.parents[p] != p {
            self.parents[p] = self.parents[self.parents[p]];
            p = self.parents[p];
        }
        p
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let root_p = self.find(p);
        let root_q = self.find(q);
        if root_p == root_q { return; }
        self.parents[root_p] = root_q;
        self.count -= 1;
    }
}

struct Solution;

impl Solution {
    pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
        let size = m.len();
        let mut circles = DisjointSet::with_capacity(size);
        for i in 0..size {
            for j in 0..i {
                if m[i][j] == 1 {
                    circles.union(i, j);
                }
            }
        }
        circles.count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 1], vec![0, 1, 1]]));
    }
}

