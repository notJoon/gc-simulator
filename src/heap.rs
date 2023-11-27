use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};

use crate::{
    free_list::FreeList,
    mem::{self, Memory},
    object::{Address, Field, Object, ObjectAddress, ObjectTrait},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Heap {
    pub roots: BTreeSet<ObjectAddress>,
    pub objects: BTreeMap<ObjectAddress, Object>,
    pub free_list: FreeList,
    pub memory: Vec<Memory>,
    pub alignment: usize,
}

pub enum HeapError {
    OutOfMemory,
    FailedToFreed,
    FailedToAllocated,
    CannotMoveObject,
    ObjectNotFound,
    SegmentFault,
}

impl Heap {
    pub fn new(size: usize, alignment: usize) -> Self {
        Self {
            roots: BTreeSet::new(),
            objects: BTreeMap::new(),
            free_list: FreeList::new(vec![(0, size)]),
            memory: vec![Memory::free(); size],
            alignment,
        }
    }

    /// Find `Object` in the heap based on the given address.
    pub fn lookup(&self, address: usize) -> Result<ObjectAddress, HeapError> {
        if let Some((_addr, obj)) = self.objects.iter().find(|(_, obj)| {
            let obj_size = obj.object_size();
            address >= obj.get_address() && address < obj.get_address() + obj_size
        }) {
            let offset = address - obj.get_address();
            let field = obj.fields.get(offset).unwrap();
            match field {
                Field::Ref(Address::Ptr(addr)) => return Ok(*addr),
                Field::Ref(Address::NullPtr) => return Err(HeapError::SegmentFault),
                _ => return Err(HeapError::SegmentFault),
            }
        }

        Err(HeapError::ObjectNotFound)
    }

    pub fn calculate_free_memory(&self) -> usize {
        self.free_list
            .inner
            .iter()
            .fold(0, |acc, (_, len)| acc + len)
    }

    pub fn merge_free_ranges(&mut self) -> Result<(), HeapError> {
        self.free_list.merge_adjacent_block();
        Ok(())
    }

    pub fn free_object(&mut self, obj: Object) -> Result<(), HeapError> {
        let addr = obj.get_address();

        if let Some(o) = self.objects.get(&addr) {
            let size = o.object_size();

            // Inserts a block into the free list,
            // merges adjacent blocks, and removes the block from the roots.
            self.free_list.insert(addr, size);
            self.free_list.merge_adjacent_block();
            self.roots.remove(&addr);

            return Ok(());
        }

        Err(HeapError::FailedToFreed)
    }

    /// Moves an object from one location in the heap to another.
    pub fn move_object(&mut self, from: usize, to: usize) -> Result<(), HeapError> {
        let obj = match self.objects.get(&from) {
            Some(o) => o.clone(),
            None => return Err(HeapError::SegmentFault),
        };

        let is_root = self.roots.contains(&from);
        self.free_object(obj.clone())?;

        if is_root {
            self.roots.insert(to);
        }

        let obj_size = obj.object_size();
        self.objects.insert(to, obj);
        self.free_list.insert(from, obj_size);
        self.free_list.merge_adjacent_block();

        Ok(())
    }

    pub fn refresh(&mut self) {
        self.reset_memory().unwrap();
        self.set_memory_cell_allocated().unwrap();
    }

    pub fn last_object(&self) -> Option<&Object> {
        self.objects.values().last()
    }

    /// Returns the address of the object that is next to the given address.
    pub fn next_object(&self, addr: usize) -> Option<&Object> {
        self.objects.values().find(|obj| obj.get_address() > addr)
    }

    pub fn prev_object(&self, addr: usize) -> Option<&Object> {
        self.objects
            .values()
            .rev()
            .find(|obj| obj.get_address() < addr)
    }

    pub fn aligned_position(&self, pos: usize) -> usize {
        if self.alignment == 0 {
            return pos;
        }

        (pos + (self.alignment - 1)) & !(self.alignment - 1)
    }

    /// Reset all the memory cells to free.
    fn reset_memory(&mut self) -> Result<(), HeapError> {
        for cell in self.memory.iter_mut() {
            cell.status = mem::Status::Free;
        }

        Ok(())
    }

    /// Set all the memory cells to allocated.
    fn set_memory_cell_allocated(&mut self) -> Result<(), HeapError> {
        for (addr, obj) in &self.objects {
            let size = obj.object_size();
            for offset in 0..size {
                match self.memory.get_mut(addr + offset) {
                    Some(cell) => cell.status = mem::Status::Allocated,
                    None => return Err(HeapError::FailedToAllocated),
                }
            }
        }

        Ok(())
    }
}

impl Debug for HeapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeapError::OutOfMemory => write!(f, "Out of memory"),
            HeapError::FailedToFreed => write!(f, "Failed to freed"),
            HeapError::FailedToAllocated => write!(f, "Failed to allocated"),
            HeapError::CannotMoveObject => write!(f, "Cannot move object"),
            HeapError::ObjectNotFound => write!(f, "Object not found"),
            HeapError::SegmentFault => write!(f, "SIGSEGV: Segmentation fault"),
        }
    }
}
