use alloc::heap;
use std::collections::HashMap;
use std::os::raw::c_void;
#[cfg(test)]
use std::ptr;
use Allocator;

#[derive(Default, Debug)]
pub struct RustAllocator {
    /// Map of alloc locations to number of bytes allocated
    allocations: HashMap<*mut c_void, usize>
}

const ALIGN: usize = 4;

impl Drop for RustAllocator {
    fn drop(&mut self) {
        for (ptr, bytes_allocated) in self.allocations.drain() {
            unsafe {
                heap::deallocate(ptr as *mut u8, bytes_allocated as usize, ALIGN);
            }
        }
    }
}

impl Allocator for RustAllocator {
    unsafe fn allocate(&mut self, old_pointer: *mut c_void, size: usize) -> *mut c_void {
        let allocation = if old_pointer.is_null() || !self.allocations.contains_key(&old_pointer) {
            heap::allocate(size as usize, ALIGN) as *mut c_void
        } else {
            let old_alloced = self.allocations.remove(&old_pointer).unwrap();
            heap::reallocate(old_pointer as *mut u8, old_alloced, size, ALIGN) as *mut c_void
        };
        self.allocations.insert(allocation, size);
        allocation
    }

    unsafe fn deallocate(&mut self, pointer: *mut c_void) {
        if let Some(bytes_allocated) = self.allocations.remove(&pointer) {
            heap::deallocate(pointer as *mut u8, bytes_allocated as usize, ALIGN);
        }
    }
}

#[test]
fn test_rust_allocation() {
    let mut allocator = RustAllocator::default();
    let alloced = unsafe {
        allocator.allocate(ptr::null_mut(), 20)
    };
    assert_eq!(*allocator.allocations.get(&alloced).unwrap(), 20);

    unsafe { allocator.deallocate(alloced) };
    assert!(allocator.allocations.get(&alloced).is_none());
}

#[test]
fn test_raw_allocation() {
    use into_raw_allocator;
    let mut allocator = RustAllocator::default();
    let alloced = unsafe {
        let raw_alloc = into_raw_allocator(&mut allocator);
        (raw_alloc.alloc.unwrap())(raw_alloc.userdata, ptr::null_mut(), 32)
    };

    assert_eq!(*allocator.allocations.get(&alloced).unwrap(), 32);

    unsafe {
        let raw_alloc = into_raw_allocator(&mut allocator);
        (raw_alloc.free.unwrap())(raw_alloc.userdata, alloced)
    }

    assert!(allocator.allocations.get(&alloced).is_none());
}
