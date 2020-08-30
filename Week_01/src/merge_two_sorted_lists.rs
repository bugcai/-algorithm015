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
    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(-1);
        let mut tail = &mut dummy;

        while let (Some(n1), Some(n2)) = (l1.as_ref(), l2.as_ref()) {
            if n1.val < n2.val {
                tail.next = l1;
                tail = tail.next.as_mut().unwrap();
                l1 = tail.next.take();
            } else {
                tail.next = l2;
                tail = tail.next.as_mut().unwrap();
                l2 = tail.next.take();
            }
        }

        tail.next = if l1.is_some() { l1 } else { l2 };
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_nodes(n: i32) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(1)));
        let mut prev = &mut head;
        for v in 2..n + 1 {
            match prev {
                Some(node) => {
                    node.next = Some(Box::new(ListNode::new(v)));
                    prev = &mut node.next;
                }
                None => {}
            }
        }
        return head;
    }

    fn as_vec(mut l: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = Vec::new();
        while let Some(n) = l.as_ref() {
            v.push(n.val);
            l = l.as_mut().unwrap().next.take();
        }
        v
    }

    #[test]
    fn case() {
        let l1 = init_nodes(3);
        let l2 = init_nodes(5);
        assert_eq!(as_vec(Solution::merge_two_lists(l1, l2)), vec![1, 1, 2, 2, 3, 3, 4, 5]);
    }
}