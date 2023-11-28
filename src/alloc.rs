use std::fmt::Debug;

use crate::object::ObjectTrait;
use crate::{
    free_list::FreeList,
    heap::Heap,
    mem::{Memory, Status},
    object::{Object, ObjectAddress},
};

#[derive(Debug, Default)]
pub struct Allocator {}

pub enum AllocatorError {
    FailedToAllocated,
}

impl Allocator {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn allocate(
        &mut self,
        heap: &mut Heap,
        object: Object,
        is_root: bool,
    ) -> Result<ObjectAddress, AllocatorError> {
        let size = object.size();

        if let Some(alignment_start) = self.find_free_block(heap, size) {
            heap.objects.insert(alignment_start, object);
            if is_root {
                heap.roots.insert(alignment_start);
            }

            // Update memory status
            for i in alignment_start..alignment_start + size {
                if let Some(memory_block) = heap.memory.get_mut(i) {
                    memory_block.status = Status::Allocated;
                }
            }

            return Ok(alignment_start);
        }

        Err(AllocatorError::FailedToAllocated)
    }

    pub fn find_free_block(&mut self, heap: &mut Heap, size: usize) -> Option<ObjectAddress> {
        for (block_start, block_size) in heap.free_list.to_vec() {
            let aligned_start = heap.aligned_position(block_start);
            let block_end = aligned_start + size;

            if block_end <= block_start + block_size {
                self.split_block(heap, block_start, block_size, aligned_start, block_end);
                return Some(block_start);
            }
        }

        None
    }

    pub fn split_block(
        &mut self,
        heap: &mut Heap,
        block_start: ObjectAddress,
        block_size: usize,
        aligned_start: ObjectAddress,
        block_end: ObjectAddress,
    ) {
        heap.free_list.remove(block_start);

        let before_size = aligned_start - block_start;
        let after_size = block_start + block_size - block_end;

        if before_size > 0 {
            heap.free_list.insert(block_start, before_size);
        }

        if after_size > 0 {
            heap.free_list.insert(block_end, after_size);
        }
    }
}

impl Debug for AllocatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AllocatorError::FailedToAllocated => write!(f, "Failed to allocated"),
        }
    }
}

// test purpose
pub fn create_free_list_heap(alignment: usize, free_list: Vec<(ObjectAddress, usize)>) -> Heap {
    Heap {
        memory: vec![Memory::new(Status::Free); 10],
        free_list: FreeList::new(free_list),
        alignment,
        ..Default::default()
    }
}
