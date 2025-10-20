use std::{
    alloc::{self, Layout},
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

#[derive(Debug)]
pub struct NDBuffer<V> {
    buffer: NonNull<V>,
    size: usize,
}

impl<V> NDBuffer<V> {
    pub fn new() -> NDBuffer<V> {
        NDBuffer {
            buffer: NonNull::dangling(),
            size: 0,
        }
    }

    pub fn deallocate(&mut self) {
        if self.size > 0 {
            let layout = Layout::array::<V>(self.size).unwrap();
            unsafe {
                alloc::dealloc(self.buffer.as_ptr() as *mut u8, layout);
            }
        }
    }

    pub fn resize(&mut self, size: usize) {
        if size > 0 {
            let layout = Layout::array::<V>(size).unwrap();
            let new_ptr = if self.size == 0 {
                unsafe { alloc::alloc(layout) }
            } else {
                let old_layout = Layout::array::<V>(self.size).unwrap();
                let old_ptr = self.buffer.as_ptr() as *mut u8;
                unsafe { alloc::realloc(old_ptr, old_layout, layout.size()) }
            };
            self.buffer = match NonNull::new(new_ptr as *mut V) {
                Some(p) => p,
                None => alloc::handle_alloc_error(layout),
            };
        } else {
            self.deallocate();
            self.buffer = NonNull::dangling();
        }
        self.size = size;
    }

    pub fn new_with_capacity(size: usize) -> NDBuffer<V> {
        let mut res = NDBuffer::<V>::new();
        res.resize(size);
        res
    }

    pub fn as_ptr(&self) -> *mut V {
        self.buffer.as_ptr()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl<V> Drop for NDBuffer<V> {
    fn drop(&mut self) {
        self.deallocate();
    }
}

impl<V> Deref for NDBuffer<V> {
    type Target = [V];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.buffer.as_ptr(), self.size) }
    }
}

impl<V> DerefMut for NDBuffer<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.buffer.as_ptr(), self.size) }
    }
}

impl<V> Default for NDBuffer<V> {
    fn default() -> Self {
        Self::new()
    }
}
