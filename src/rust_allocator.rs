use alloc::heap;
use std::os::raw::c_void;
use std::ptr;
use Allocator;

/// A nuklear compatible allocator which uses the rust allocator to allocate
/// memory. On drop, it will free all its stored memory, so ensure that it will
/// live as long as the memory it holds is needed.
#[derive(Default, Debug)]
pub struct RustAllocator {
    /// Map of alloc locations to number of bytes allocated
    allocations: Vec<ManagedPtr>
}

impl Allocator for RustAllocator {
    /// Allocate `size` amount of memory. If `old_pointer` is equivalent to `ptr::null_mut()`,
    /// Then it will create a new allocation, otherwise it will reallocate the memory at
    /// `old_pointer`.
    unsafe fn allocate(&mut self, old_pointer: *mut c_void, size: usize) -> *mut c_void {
        let mut to_append = None;
        let allocation = match self.allocations.iter_mut().find(|p| p.ptr == old_pointer) {
            Some(ptr) => {
                ptr.realloc(size);
                ptr.ptr
            }
            None if old_pointer.is_null() => {
                let new_ptr = ManagedPtr::alloc(size);
                to_append = Some(new_ptr.shallow_copy());
                new_ptr.ptr
            }
            _ => ptr::null_mut(),
        };

        // TODO(burtonageo): This should really be written inline in the match arm when
        // non-lexical lifetimes land
        if let Some(append) = to_append {
            self.allocations.push(append);
        }

        allocation
    }

    /// Deallocates the memory at `pointer` if it is owned by this allocator.
    unsafe fn deallocate(&mut self, pointer: *mut c_void) {
        if let Some(idx) = self.allocations.iter().position(|p| p.ptr == pointer) {
            self.allocations.swap_remove(idx);
        }
    }
}

/// A RAII pointer type to manage heap memory automatically.
#[derive(Debug)]
struct ManagedPtr {
    /// Pointer to the allocation
    ptr: *mut c_void,

    /// Size of the allocation
    bytes_allocated: usize
}

const ALIGN: usize = 4;

impl ManagedPtr {
    fn alloc(size: usize) -> Self {
        let allocation = unsafe { heap::allocate(size, ALIGN) };
        ManagedPtr {
            ptr: allocation as *mut _,
            bytes_allocated: size
        }
    }

    fn realloc(&mut self, new_size: usize) {
        self.ptr = unsafe {
            heap::reallocate(self.ptr as *mut _, self.bytes_allocated, new_size, ALIGN) as *mut _
        };
        self.bytes_allocated = new_size;
    }

    fn shallow_copy(&self) -> Self {
        ManagedPtr {
            ptr: self.ptr,
            bytes_allocated: self.bytes_allocated
        }
    }
}

impl Drop for ManagedPtr {
    fn drop(&mut self) {
        unsafe {
            heap::deallocate(self.ptr as *mut u8, self.bytes_allocated, ALIGN);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::os::raw::c_void;
    use std::ptr;
    use super::*;
    use ::Allocator;

    impl RustAllocator {
        fn get_allocation_size(&self, ptr: &*mut c_void) -> Option<usize> {
            self.allocations.iter().find(|p| p.ptr == *ptr).map(|p| p.bytes_allocated)
        }
    }

    #[test]
    fn test_rust_allocation() {
        let mut allocator = RustAllocator::default();
        let alloced = unsafe {
            allocator.allocate(ptr::null_mut(), 20)
        };

        assert_eq!(allocator.get_allocation_size(&alloced).unwrap(), 20);
        unsafe { allocator.deallocate(alloced) };
        assert!(allocator.get_allocation_size(&alloced).is_none());
    }
    
    #[test]
    fn test_raw_allocation() {
        use into_raw_allocator;
        let mut allocator = RustAllocator::default();
        let alloced = unsafe {
            let raw_alloc = into_raw_allocator(&mut allocator);
            (raw_alloc.alloc.unwrap())(raw_alloc.userdata, ptr::null_mut(), 32)
        };
    
        assert_eq!(allocator.get_allocation_size(&alloced).unwrap(), 32);
    
        unsafe {
            let raw_alloc = into_raw_allocator(&mut allocator);
            (raw_alloc.free.unwrap())(raw_alloc.userdata, alloced)
        }
    
        assert!(allocator.get_allocation_size(&alloced).is_none());
    }
}