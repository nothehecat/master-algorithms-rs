#!/usr/bin/env run-cargo-script

struct CircularQueue {
    buf: Vec<i32>,
    start: i32,
    end: i32,
    capacity: i32,
    size: i32,
}

impl CircularQueue {
    fn new(k: i32) -> Self {
        Self {
            buf: vec![0; k as usize],
            start: 0,
            end: 0,
            capacity: k,
            size: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.buf[self.end as usize] = value;
        self.end = (self.end + 1) % self.capacity;
        self.size += 1;
        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.start = (self.start + 1) % self.capacity;
        self.size -= 1;
        true
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.buf[self.start as usize]
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let i = ((self.end - 1) % self.capacity + self.capacity) % self.capacity;
        self.buf[i as usize]
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}
