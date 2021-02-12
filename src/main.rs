struct RingBuffer<T> {
    data_: Box<[T]>
}

impl<T> RingBuffer<T> {

    fn with_capacity(n : usize) -> RingBuffer<T> {
        todo!()
    }

    fn push(val: T) {
        todo!()
    }

    fn pop() {
        todo!()
    }

}

impl<T> IntoIterator for RingBuffer<T> {
    type Item=T;
    type IntoIter=std::vec::IntoIter<T>;

    fn into_iter(self) -> <Self as std::iter::IntoIterator>::IntoIter {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
