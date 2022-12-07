#[warn(dead_code)]
#[derive(Debug, Default)]
pub struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { queue: vec![] }
    }

    pub fn enqueue(&mut self, item: T) {
        self.queue.push(item)
    }

    pub fn dequeue(&mut self) -> T {
        self.queue.remove(0)
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        self.queue.first()
    }

    pub fn peek_mut(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.dequeue())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn it_works() {
        let mut queue: Queue<i32> = Queue::new();
        queue.enqueue(42);

        assert_eq!(1, queue.len());
        assert_eq!(42, queue.dequeue());
        assert_eq!(0, queue.len());

        for i in 0..10_000 {
            queue.enqueue(i)
        }

        assert_eq!(10_000, queue.len());
        for i in 0..10_000 {
            assert_eq!(i, queue.dequeue())
        }

        println!("{:?}", queue)
    }
}
