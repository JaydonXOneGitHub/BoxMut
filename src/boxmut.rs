use std::alloc::{Layout, alloc, dealloc};

pub struct BoxMut<T>
where
    T: ?Sized,
{
    handle: *mut T,
    layout: Layout,
}

impl<T: Sized> BoxMut<T> {
    pub fn new(value: T) -> Self {
        return unsafe {
            let layout: Layout = Layout::new::<T>();

            let ptr: *mut T = alloc(layout) as *mut T;

            if !ptr.is_null() {
                std::ptr::write(ptr, value);
            }

            Self {
                handle: ptr,
                layout: layout,
            }
        };
    }

    pub fn is_valid(&self) -> bool {
        return !self.handle.is_null();
    }

    pub fn get_ref(&self) -> Option<&T> {
        return unsafe { self.handle.as_ref() };
    }

    pub fn get_mut(&self) -> Option<&mut T> {
        return unsafe { self.handle.as_mut() };
    }
}

impl<T: Sized + Clone> Clone for BoxMut<T> {
    fn clone(&self) -> Self {
        return unsafe {
            let layout: Layout = Layout::new::<T>();

            let ptr: *mut T = alloc(layout) as *mut T;

            if !ptr.is_null() {
                std::ptr::write(ptr, self.get_ref().unwrap().clone());
            }

            Self {
                handle: ptr,
                layout: layout,
            }
        };
    }
}

impl<T: ?Sized> Drop for BoxMut<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self.handle);
            dealloc(self.handle as *mut u8, self.layout);
        }
    }
}
