struct RingBuffer<T> {
    data_: Vec<Option<T>>,
    head_: usize,
    len_: usize
}

impl<T> std::ops::Index<usize> for RingBuffer<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        self.data_[self.head_].as_ref().unwrap()
    }

}

impl<T: Clone> RingBuffer<T> {

    fn with_capacity(n : usize) -> RingBuffer<T> {
        RingBuffer { data_ : vec![None; n], head_: 0, len_: 0 }
    }

    fn push(&mut self, val: T) {
        if self.len_ == self.data_.len() {
            // let mut new_data = Vec::<T>::with_capacity(self.len_ * 2 + 1);
            // for elem in self.into_iter() {

            // }
        }
    }

    fn pop(&mut self) {
        todo!()
    }

}

struct RingBufferIterator<'a, T> {
    ring_buffer_: &'a RingBuffer<T>,
    pos_: usize
}

impl<'a, T> Iterator for RingBufferIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos_ < self.ring_buffer_.len_ {
            self.pos_ += 1;
            Some(&self.ring_buffer_[self.pos_])
        } else {
            None
        }
    }
}

impl<'a, T> IntoIterator for &'a RingBuffer<T> {
    type Item = &'a T;
    type IntoIter = RingBufferIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        RingBufferIterator { ring_buffer_: self, pos_: 0 }
    }
}

fn main() {
    println!("Hello, world!");
}
