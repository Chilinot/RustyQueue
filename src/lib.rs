pub struct RustyQueue<T> {
    queue: Vec<T>,
}

impl<T> RustyQueue<T> {
    
    /// Construct a new empty queue.
    pub fn new() -> RustyQueue<T> {
        RustyQueue {
            queue: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.queue.push(value);
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.queue.is_empty() {
            Ok(self.queue.remove(0))
        }
        else {
            Err("The queue is empty!")
        }
    }
}

#[cfg(test)]
mod tests {

    use super::RustyQueue;

    #[test]
    fn basic() {
        let mut queue: RustyQueue<i32> = RustyQueue::new();

        queue.enqueue(12);
        queue.enqueue(13);
        queue.enqueue(11);

        assert!(12 == queue.dequeue().unwrap());
        assert!(13 == queue.dequeue().unwrap());
        assert!(11 == queue.dequeue().unwrap());
    }

    #[test]
    fn dequeue_empty() {
        let mut queue: RustyQueue<i32> = RustyQueue::new();

        let result = queue.dequeue();

        match result {
            Result::Ok(_) => panic!(),
            Result::Err(e) => assert_eq!("The queue is empty!", e),
        }
    }

    #[test]
    #[should_panic]
    fn dequeue_too_much() {
        let mut queue: RustyQueue<i32> = RustyQueue::new();

        queue.enqueue(12);

        assert!(12 == queue.dequeue().unwrap());
        queue.dequeue().unwrap(); // This should throw a panic!
    }
}
