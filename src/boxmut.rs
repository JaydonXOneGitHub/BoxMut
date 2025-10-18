use std::alloc::{Layout, alloc, dealloc};

pub struct BoxMut<T>
where
    T: ?Sized,
{
    handle: *mut T,
    layout: Layout,
}

impl<T: Sized> BoxMut<T> {
    pub fn new(value: T) -> Result<Self, String> {
        return unsafe {
            let layout: Layout = Layout::new::<T>();

            let ptr: *mut T = alloc(layout) as *mut T;

            if ptr.is_null() {
                Result::Err("Allocation failed!".into())
            } else {
                std::ptr::write(ptr, value);

                Result::Ok(Self {
                    handle: ptr,
                    layout: layout,
                })
            }
        };
    }

    pub fn get_ref(&self) -> Option<&T> {
        return unsafe { self.handle.as_ref() };
    }

    pub fn get_mut(&self) -> Option<&mut T> {
        return unsafe { self.handle.as_mut() };
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
