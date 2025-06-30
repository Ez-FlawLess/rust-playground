use std::marker::PhantomData;
use std::ops::Deref;

struct VoidPtr<T>(*mut std::ffi::c_void, PhantomData<T>);

impl<T> VoidPtr<T> {
    fn new(x: T) -> Self {
        let x = Box::into_raw(Box::new(x));
        Self(x as *mut std::ffi::c_void, PhantomData)
    }
}

impl<T> Drop for VoidPtr<T> {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.0));
        }
    }
}

impl<T> Deref for VoidPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(self.0 as *const T)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_void_ptr() {
        let data = VoidPtr::new(42);

        let num = *data; // -> *(data.deref())

        assert_eq!(num, 42);
    }
}