#![allow(dead_code)]
#![allow(unused)]

/// The dequeue in data structure stands for Double Ended Queue
#[derive(Debug)]
pub struct Dequeue<T> {
    pub size: usize,
    pub capacity: usize,

    buffer: Vec<T>,
    head_index: i32,
    tail_index: i32,
}

impl<T> Dequeue<T> {
    #![allow(clippy::uninit_vec)]
    fn new() -> Self {
        let capacity = 64;
        let mut vect = Vec::with_capacity(capacity);
        unsafe {
            vect.set_len(capacity);
        }

        Self {
            capacity,
            size: 0,
            buffer: vect,
            head_index: 0,
            tail_index: 0,
        }
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    /// This function is used to check whether the deque in data structure is empty or not.
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// This function is used to insert element at the front end of the deque in data structure
    fn add_to_head(&mut self, item: T) {
        if self.is_full() {
            panic!("Oooops, the deque is full")
        } else {
            if self.head_index == 0 {
                self.head_index = (self.capacity as i32) - 1;
            } else {
                self.head_index -= 1;
            }

            self.buffer[self.head_index as usize] = item;
            self.size += 1;
        }
    }

    /// This function is used to insert element at the rear end of the deque in data structure
    fn add_to_tail(&mut self, item: T) {
        if self.is_full() {
            panic!("Oooops, the deque is full")
        } else {
            if self.tail_index == ((self.capacity - 1) as i32) {
                self.tail_index = 0;
            } else {
                self.tail_index += 1;
            }

            self.buffer[self.tail_index as usize] = item;
            self.size += 1;
        }
    }

    /// This function is used to delete elements from the front end of the deque in data structure
    fn remove_from_head(&mut self) {
        if self.is_empty() {
            panic!("An empty dequeue")
        } else {
            if self.head_index == self.tail_index {
                // TODO
            } else if self.head_index == ((self.size - 1) as i32) {
                self.head_index = 0;
            } else {
                self.head_index += 1;
            }

            self.size -= 1;
        }
    }

    /// This function is used to delete elements from the rear end of the deque in data structure
    fn remove_from_tail(&mut self) {
        if self.is_empty() {
            panic!("An empty dequeue")
        } else {
            if self.tail_index == 0 {
                self.tail_index = (self.capacity - 1) as i32;
            } else {
                self.tail_index -= 1;
            }

            self.size -= 1;
        }
    }

    /// This function is used to get or peek elements from the front end of the deque in data structure
    fn get_head(&mut self) -> Option<T> {
        // if self.is_empty() {
        //     None
        // } else {
        //     let r = Some(self.buffer.remove(self.head_index as usize));
        //     self.remove_from_head();
        //     r
        // }
        todo!()
    }

    /// This function is used to get or peek element from the rear end of the deque in data structure
    fn get_tail(&mut self) -> Option<T> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Dequeue;

    #[test]
    fn add_to_head() {
        let mut dequeue: Dequeue<i32> = Dequeue::new();

        for i in 0..dequeue.capacity {
            dequeue.add_to_head(i as i32);
        }
    }

    #[test]
    #[should_panic]
    fn add_to_head_in_full_state_must_panic() {
        let mut dequeue: Dequeue<i32> = Dequeue::new();

        for i in 0..=dequeue.capacity {
            dequeue.add_to_head(i as i32);
        }
    }

    #[test]
    fn add_to_tail() {
        let mut dequeue: Dequeue<usize> = Dequeue::new();

        for i in 0..dequeue.capacity {
            dequeue.add_to_tail(i);
        }
    }

    #[test]
    #[should_panic]
    fn add_to_tail_in_full_state_must_panic() {
        let mut dequeue: Dequeue<usize> = Dequeue::new();

        for i in 0..=dequeue.capacity {
            dequeue.add_to_tail(i);
        }
    }

    #[test]
    fn remove_from_head() {
        let mut dequeue: Dequeue<usize> = Dequeue::new();

        dequeue.add_to_head(1);
        assert_eq!(1, dequeue.size);
        dequeue.remove_from_head();
        assert_eq!(0, dequeue.size);
    }

    #[test]
    #[should_panic]
    fn remove_from_head_in_empty_state_must_panic() {
        let mut dequeue: Dequeue<usize> = Dequeue::new();
        dequeue.remove_from_head()
    }

    #[test]
    fn remove_from_tail() {
        let mut dequeue: Dequeue<usize> = Dequeue::new();

        dequeue.add_to_tail(1);
        assert_eq!(1, dequeue.size);
        dequeue.remove_from_tail();
        assert_eq!(0, dequeue.size);
    }

    #[test]
    #[should_panic]
    fn remove_from_tail_in_empty_state_must_panic() {
        let mut dequeue: Dequeue<usize> = Dequeue::new();
        dequeue.remove_from_tail();
    }

    #[test]
    fn get_head() {
        // let mut dequeue: Dequeue<usize> = Dequeue::new();

        // assert_eq!(None, dequeue.get_head());
        // dequeue.add_to_head(1);
        // assert_eq!(Some(1), dequeue.get_head());
        // dequeue.add_to_head(2);
        // dequeue.add_to_head(3);
        // assert_eq!(Some(3), dequeue.get_head());
    }
}
