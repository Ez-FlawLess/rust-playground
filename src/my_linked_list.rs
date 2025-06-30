use std::alloc::Layout;
use std::mem::offset_of;
use std::{alloc, mem, ptr};

pub struct MyLinkedList<T>{
    head: Option<MyNode<T>>,
}

pub struct MyNode<T>{
    data: T,
    next: Option<Box<MyNode<T>>>,
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

    pub fn pop(&mut self) -> Option<T> {
        match self.head {
            None => None,
            Some(ref mut node) if node.next.is_some() => {
                node.pop_next()
            },
            _ => {
                let mut head = self.head.take().unwrap();

                let node_ptr = &mut head as *mut MyNode<T> as *mut _;

                let data = unsafe {
                    let addr = (node_ptr as usize) + offset_of!(MyNode<T>, data);

                    ptr::read(addr as *mut T)
                };

                mem::forget(head);

                Some(data)
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.head.as_ref()
            .map(|node| node.get(index))?
    }
}

impl<T> MyNode<T> {
    fn new(data: T) -> Self {
        Self{
            data,
            next: None,
        }
    }

    fn push_next(&mut self, data: T) {
        match self.next {
            None => {
                self.next = Some(Box::new(Self::new(data)));
            },
            Some(ref mut node) => {
                node.push_next(data);
            },
        }
    }

    fn pop_next(&mut self) -> Option<T> {
        match self.next {
            None => unreachable!("INTERNAL ERROR"),
            Some(ref mut node) if node.next.is_some() => {
                node.pop_next()
            },
            _ => {
                let node = self.next.take().unwrap();

                let node_ptr = Box::into_raw(node);

                let data = unsafe {
                    let addr = (node_ptr as usize) + offset_of!(MyNode<T>, data);

                    ptr::read(addr as *mut T)
                };

                let layout = Layout::new::<MyNode<T>>();
                unsafe {
                    alloc::dealloc(node_ptr as *mut _, layout);
                }

                Some(data)
            },
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index == 0 {
            return Some(&self.data);
        }

        self.next.as_ref()
            .map(|node| {
                node.get(index - 1)
            })?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_my_linked_list() {
        let mut list = MyLinkedList::new();

        assert_eq!(list.get(0), None);
        
        list.push(1);
        list.push(2);
        list.push(3);
        
        assert_eq!(list.get(0), Some(&1));
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.get(2), Some(&3));
        assert_eq!(list.get(3), None);
        
        for i in (1..=3).rev() {
            let data = list.pop().unwrap();
            assert_eq!(data, i);
        }
    }

    #[test]
    fn test_my_linked_list_pop_refs_still_valid() {
        let mut my_vec = MyLinkedList::new();

        my_vec.push(Box::new(1));
        my_vec.push(Box::new(2));
        my_vec.push(Box::new(3));

        for i in (1..=3).rev() {
            let item = my_vec.pop().unwrap();
            assert_eq!(*item, i);
        }
    }
}