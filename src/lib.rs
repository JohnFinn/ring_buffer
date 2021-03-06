#[derive(Debug)]
pub struct RingBuffer<T> {
    data_: Vec<Option<T>>,
    head_: usize,
    len_: usize
}

impl<T> std::ops::Index<usize> for RingBuffer<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        self.index_(idx).as_ref().unwrap()
    }
}

impl<T> RingBuffer<T> {

    pub fn with_capacity(n : usize) -> RingBuffer<T> {
        let mut data = Vec::<Option<T>>::new();
        data.resize_with(n, || None);
        RingBuffer { data_ : data, head_: 0, len_: 0 }
    }

    fn index_mut_(&mut self, idx: usize) -> &mut Option<T> {
        let capacity = self.data_.len();
        &mut self.data_[(self.head_ + idx) % capacity]
    }

    fn index_(&self, idx: usize) -> &Option<T> {
        let capacity = self.data_.len();
        &self.data_[(self.head_ + idx) % capacity]
    }

    fn resize2x_(&mut self) {
        let new_size = if self.data_.len() > 0 { self.len_ * 2 } else { 1 };
        let mut new_data = Vec::<Option<T>>::with_capacity(new_size);
        for idx in 0..self.len_ {
            // hope this would avoid expensive copying but not sure
            let mut elem : Option<T> = None;
            std::mem::swap(self.index_mut_(idx), &mut elem);
            new_data.push(elem);
        }
        new_data.resize_with(new_size, || None);
        // TODO make sure this doesn't copy anything
        self.data_ = new_data;
        self.head_ = 0;
    }

    pub fn push_back(&mut self, val: T) {
        if self.len_ == self.data_.len() {
            self.resize2x_();
        }
        let mut val = Some(val);
        std::mem::swap(self.index_mut_(self.len_), &mut val);
        self.len_ += 1;
    }

    pub fn push_front(&mut self, val: T) {
        if self.len_ == self.data_.len() {
            self.resize2x_();
        }
        let mut val = Some(val);
        self.head_ = if self.head_ > 0 { self.head_ - 1 } else { self.data_.len() - 1 };
        std::mem::swap(self.index_mut_(0), &mut val);
        self.len_ += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.len_ == 0 {
            None
        } else {
            let mut result = None;
            std::mem::swap(self.index_mut_(self.len_ - 1), &mut result);
            self.len_ -= 1;
            result
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len_ == 0 {
            None
        } else {
            let mut result = None;
            std::mem::swap(self.index_mut_(0), &mut result);
            self.len_ -= 1;
            self.head_ = (self.head_ + 1) % self.data_.len();
            result
        }
    }
}

pub struct RingBufferIterator<'a, T> {
    ring_buffer_: &'a RingBuffer<T>,
    pos_: usize
}

impl<'a, T> Iterator for RingBufferIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos_ < self.ring_buffer_.len_ {
            let res = Some(&self.ring_buffer_[self.pos_]);
            self.pos_ += 1;
            res
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


#[cfg(test)]
mod tests {
    use super::*;
    extern crate rand;

    #[test]
    fn it_works() {
        let mut a = std::collections::VecDeque::<u8>::new();
        let mut b : RingBuffer<u8> = RingBuffer::with_capacity(0);

        for _ in 0..10000 {
            let action = rand::random::<u8>();

            if action % 4 == 0 {
                let val = rand::random::<u8>();
                a.push_back(val);
                b.push_back(val);
            } else if action % 4 == 1 {
                let val = rand::random::<u8>();
                a.push_front(val);
                b.push_front(val);
            } else if action % 4 == 2 {
                let v1 = a.pop_back();
                let v2 = b.pop_back();
                assert_eq!(v1, v2);
            } else if action % 4 == 3 {
                let v1 = a.pop_front();
                let v2 = b.pop_front();
                assert_eq!(v1, v2);
            }
            assert!(Iterator::eq(a.iter(), b.into_iter()));
        }
    }
}
