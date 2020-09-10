// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution;

impl Solution {
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        fn helper(node: &Option<Box<ListNode>>, result: &mut Vec<i32>) {
            if let Some(n) = node.as_deref() {
                helper(&n.next, result);
                result.push(n.val);
            }
        }
        let mut result: Vec<i32> = vec![];
        helper(&head, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {}
}

