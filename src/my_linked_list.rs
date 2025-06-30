use std::ptr::null_mut;

pub struct MyLinkedList<T>{
    head: Option<MyNode<T>>,
}

pub struct MyNode<T>{
    data: T,
    next: *mut MyNode<T>,
}

impl<T> MyLinkedList<T> {
    pub fn new() -> Self {
        Self{
            head: None,
        }
    }

    pub fn push(&mut self, data: T) {
        match self.head {
            None => {
                self.head = Some(MyNode::new(data));
            },
            Some(ref mut node) => {
                node.push_next(data);
            },
        };
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        match self.head {
            None => None,
            Some(ref node) => {
                node.get(index)
            }
        }
    }
}

impl<T> Drop for MyLinkedList<T> {
    fn drop(&mut self) {
        if let Some(node) = self.head.take() {
            drop(node);
        }
    }
}

impl<T> MyNode<T> {
    fn new(data: T) -> Self {
        Self{
            data,
            next: null_mut(),
        }
    }

    fn push_next(&mut self, data: T) {
        if self.next.is_null() {
            self.next = Box::into_raw(Box::new(Self::new(data)));
        } else {
            unsafe {
                (*self.next).push_next(data);
            }
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index == 0 {
            return Some(&self.data);
        }

        if self.next.is_null() {
            return None;
        }

        unsafe {
            (*self.next).get(index - 1)
        }
    }
}

impl<T> Drop for MyNode<T> {
    fn drop(&mut self) {
        if !self.next.is_null() {
            unsafe {
                drop(Box::from_raw(self.next));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_my_linked_list() {
        let mut list = MyLinkedList::new();
        
        list.push(1);
        list.push(2);
        list.push(3);
        
        assert_eq!(list.get(0), Some(&1));
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.get(2), Some(&3));
    }
}