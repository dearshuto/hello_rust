#![feature(allocator_api)]

use std::{
    alloc::{AllocError, Allocator},
    ptr::{slice_from_raw_parts_mut, NonNull},
    sync::{Arc, Mutex},
};

struct PlacementBuffer {
    buffer: Vec<u8>,
    current_head: usize,
}

struct CustomAllocator {
    buffer: Arc<Mutex<PlacementBuffer>>,
}

impl CustomAllocator {
    pub fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(PlacementBuffer {
                buffer: Vec::with_capacity(1024),
                current_head: 0,
            })),
        }
    }
}

unsafe impl Allocator for CustomAllocator {
    fn allocate(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<std::ptr::NonNull<[u8]>, std::alloc::AllocError> {
        let size = layout.size();
        let mut buffer = self.buffer.lock().unwrap();

        // 空き容量がなくなってたらエラー
        let capacity = buffer.buffer.capacity() - buffer.current_head;
        if capacity < size {
            return Err(AllocError);
        }

        println!("Allocate: {:?}", layout);

        // 使用した領域を進める
        buffer.current_head += size;
        buffer.buffer.as_mut_ptr();
        let slice = slice_from_raw_parts_mut(buffer.buffer.as_mut_ptr(), size);
        Ok(NonNull::<[u8]>::new(slice).unwrap())
    }

    unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {
        // TODO: とりあえず消さないで一方向
    }
}

fn main() {
    let mut vec: Vec<u32, CustomAllocator> = Vec::with_capacity_in(1, CustomAllocator::new());
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("{:?}", vec);
}
