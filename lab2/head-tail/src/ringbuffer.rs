use std::collections::VecDeque;

pub struct RingBuffer<T> {
    data: VecDeque<T>,
    size: usize,
}

impl<T> RingBuffer<T> {
    fn push(&mut self, value: T) {
        if self.data.len() >= self.size {
            self.data.pop_front();
        }

        self.data.push_back(value);
    }

    pub fn from_iter(iter: impl Iterator<Item = T>, size: usize) -> Self {
        let mut ring_buffer = Self {
            data: VecDeque::new(),
            size,
        };

        ring_buffer.data.reserve(size);

        for value in iter {
            ring_buffer.push(value);
        }

        ring_buffer
    }

    pub fn into_vec(self) -> Vec<T> {
        Vec::from(self.data)
    }
}
