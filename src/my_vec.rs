use std::alloc::{alloc, dealloc, realloc, Layout};
use std::ptr;

pub struct MyVec<T> {
    ptr: *mut T,
    capacity: usize,
    len: usize,
}

const DEFAULT_CAPACITY: usize = 5;

impl<T> MyVec<T> {

    fn initial_layout() -> Layout {
        Layout::array::<T>(DEFAULT_CAPACITY).unwrap()
    }

    pub fn new() -> Self {

        let layout = Self::initial_layout();

        let ptr = unsafe {
            alloc(layout)
        };

        if ptr.is_null() {
            panic!("Failed to allocate memory");
        }

        Self{
            ptr: ptr as *mut T,
            capacity: DEFAULT_CAPACITY,
            len: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.len == self.capacity {
            let new_capacity = self.capacity * 2;

            let new_layout = Layout::array::<T>(new_capacity).unwrap();

            let ptr = unsafe {
                realloc(self.ptr as *mut u8, Self::initial_layout(), new_layout.size())
            };

            if ptr.is_null() {
                panic!("Failed to reallocate memory");
            }

            self.ptr = ptr as *mut T;
            self.capacity = new_capacity;
        }

        unsafe {
            let ptr = self.ptr.add(self.len);
            *ptr = item;
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let item = unsafe {
            let ptr = self.ptr.add(self.len - 1);
            ptr::read(ptr)
        };

        self.len -= 1;
        Some(item)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        let ptr = unsafe {
            self.ptr.add(index)
        };

        if ptr.is_null() {
            None
        } else {
            Some(unsafe{ &*ptr })
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        let layout = Layout::array::<T>(self.capacity).unwrap();

        unsafe {
            dealloc(self.ptr as *mut u8, layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_vec() {
        let mut my_vec = MyVec::<usize>::new();

        for i in 0..=10 {
            assert_eq!(my_vec.get(i), None);
            assert_eq!(my_vec.len(), i);
            my_vec.push(i);
        }

        for i in 0..=10 {
            assert_eq!(my_vec.get(i), Some(&i));
        }

        for i in (0..=10).rev() {
            assert_eq!(my_vec.pop(), Some(i));
            assert_eq!(my_vec.get(i), None);
            assert_eq!(my_vec.len(), i);
        }
        
        assert_eq!(my_vec.pop(), None);
    }
}