use std::collections::{btree_map, BTreeMap};

#[derive(Debug, PartialEq, Clone, Default)]
/// Free list is a data structure used in a dynamic memory allocator to keep track of which memory
/// blocks are free and which are allocated.
pub struct FreeList {
    pub inner: BTreeMap<usize, usize>,
}

pub struct FreeListIter<'a> {
    inner_iter: btree_map::Iter<'a, usize, usize>,
}

impl<'a> Iterator for FreeListIter<'a> {
    type Item = (&'a usize, &'a usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next()
    }
}

impl FreeList {
    pub fn new(blocks: Vec<(usize, usize)>) -> Self {
        let inner = blocks.into_iter().collect::<BTreeMap<_, _>>();
        Self { inner }
    }

    pub fn iter(&self) -> FreeListIter {
        FreeListIter {
            inner_iter: self.inner.iter(),
        }
    }

    pub fn insert(&mut self, start: usize, size: usize) {
        match self.inner.get(&start) {
            Some(&len) => {
                self.inner.insert(start, usize::max(size, len));
            }
            None => {
                self.inner.insert(start, size);
            }
        }

        self.merge_adjacent_block();
    }

    pub fn remove(&mut self, start: usize) {
        self.inner.remove(&start);
    }

    pub fn merge_adjacent_block(&mut self) {
        let mut current = self.inner.keys().cloned().next();

        // Iterates through 'self.inner' to merge adjacent or overlapping blocks
        while let Some(start) = current {
            let current_length = *self.inner.get(&start).unwrap();
            let current_end = start + current_length;

            // Find the next block that starts after 'start'
            if let Some((&next_start, &next_length)) = self.inner.range((start + 1)..).next() {
                // Check if the next block overlaps or is adjacent to the current block
                if next_start <= current_end {
                    // Calculate the new length for the merged block
                    let new_length = current_length + next_length + next_start - current_end;
                    // Update the current block's length to the new length
                    *self.inner.get_mut(&start).unwrap() = new_length;
                    // Remove the next block as it's now merged
                    self.inner.remove(&next_start);
                } else {
                    // Move to the next block if there's no overlap
                    current = Some(next_start);
                }
            } else {
                // Exit the loop if no more blocks are found
                break;
            }
        }
    }

    #[allow(clippy::unnecessary_to_owned)]
    pub fn to_vec(&self) -> Vec<(usize, usize)> {
        self.inner.to_owned().into_iter().collect()
    }
}
