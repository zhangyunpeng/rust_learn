use std::ops::{Deref, DerefMut};
use std::ptr::{NonNull, self};
use std::alloc::{self, Layout};


pub struct SVec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize
}

impl<T> SVec<T> {
    pub fn new() -> SVec<T> {
        SVec { ptr: NonNull::dangling(), len: 0, cap: 0 }
    }
    fn grow(&mut self) {
        // 计算新cap 以及需要的内存
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = self.cap * 2;
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        // 分配内存
        let new_ptr = if self.cap == 0 {
            unsafe {
                alloc::alloc(new_layout)
            }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe {
                alloc::realloc(old_ptr, old_layout, new_layout.size())
            }
        };

        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout)
        };
        self.cap = new_cap
    }

    pub fn push(&mut self, elem: T) {
        if self.cap == self.len {
            self.grow()
        } 
        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), elem)
        }
        self.len += 1;
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                Some(ptr::read(self.ptr.as_ptr().add(self.len)))
            }
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(index <= self.len, "index out of range");
        if self.len == self.cap {
            self.grow();
        }
        unsafe {
            ptr::copy(
                self.ptr.as_ptr().add(index),
                self.ptr.as_ptr().add(index + 1), 
                self.len - index
            );
            ptr::write(self.ptr.as_ptr().add(index), elem);
            self.len -= 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index <= self.len, "index out of range");
        unsafe {
            self.len -= 1;
            let result = ptr::read(self.ptr.as_ptr().add(index));
            ptr::copy(
                self.ptr.as_ptr().add(index + 1),
                self.ptr.as_ptr().add(index), 
                self.len - index
            );
            result
        }
    }
}

impl<T> Drop for SVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while let Some(_) = self.pop() {};
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout)
            }
        }
    }
}

impl<T> Deref for SVec<T>  {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe {
            std::slice::from_raw_parts(self.ptr.as_ptr(), self.len)
        }
    }
}

impl<T> DerefMut for SVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr.as_mut(), self.len)
        }
    }
}

