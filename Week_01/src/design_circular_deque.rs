struct MyCircularDeque {
    front: i32,
    rear: i32,
    capacity: i32,
    arr: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    /** Initialize your data structure here. Set the size of the deque to be k. */
    fn new(k: i32) -> Self {
        Self {
            front: 0,
            rear: 0,
            capacity: k + 1,
            arr: vec![0; (k + 1) as usize],
        }
    }

    /** Adds an item at the front of Deque. Return true if the operation is successful. */
    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        self.front = (self.front - 1 + self.capacity) % self.capacity;
        self.arr[self.front as usize] = value;
        return true;
    }

    /** Adds an item at the rear of Deque. Return true if the operation is successful. */
    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.arr[self.rear as usize] = value;
        self.rear = (self.rear + 1) % self.capacity;
        return true;
    }

    /** Deletes an item from the front of Deque. Return true if the operation is successful. */
    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.front = (self.front + 1) % self.capacity;
        return true;
    }

    /** Deletes an item from the rear of Deque. Return true if the operation is successful. */
    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.rear = (self.rear - 1 + self.capacity) % self.capacity;
        return true;
    }

    /** Get the front item from the deque. */
    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        return self.arr[self.front as usize];
    }

    /** Get the last item from the deque. */
    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        return self.arr[((self.rear - 1 + self.capacity) % self.capacity) as usize];
    }

    /** Checks whether the circular deque is empty or not. */
    fn is_empty(&self) -> bool {
        self.front == self.rear
    }

    /** Checks whether the circular deque is full or not. */
    fn is_full(&self) -> bool {
        (self.rear + 1) % self.capacity == self.front
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        let mut deque = MyCircularDeque::new(3); // 设置容量大小为3
        assert!(deque.insert_last(1));                // 返回 true
        assert!(deque.insert_last(2));                // 返回 true
        assert!(deque.insert_front(3));               // 返回 true
        assert!(!deque.insert_front(4));              // 已经满了，返回 false
        assert_eq!(deque.get_rear(), 2);                    // 返回 2
        assert!(deque.is_full());                           // 返回 true
        assert!(deque.delete_last());                       // 返回 true
        assert!(deque.insert_front(4));               // 返回 true
        assert_eq!(deque.get_front(), 4);                   // 返回 4
    }
}