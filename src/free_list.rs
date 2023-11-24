use std::collections::{BTreeMap, btree_map};

#[derive(Debug, PartialEq, Clone, Default)]
/// Free list is a data structure used in a dynamic memory allocator to keep track of which memory
/// blocks are free and which are allocated.
pub struct FreeList {
    pub inner: BTreeMap<usize, usize>
}

pub struct FreeListIter<'a> {
    inner_iter: btree_map::Iter<'a, usize, usize>
}

impl<'a> Iterator for FreeListIter<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter
            .next()
            .map(|(k, v)| (*k, *v))
    }
}

impl FreeList {
    pub fn new(blocks: Vec<(usize, usize)>) -> Self {
        let inner = blocks.into_iter().collect::<BTreeMap<_, _>>();
        Self { inner }
    }

    pub fn insert(&mut self, start: usize, end: usize) {
        if let Some(&len) = self.inner.get(&start) {
            self.inner.insert(start, usize::max(len, end));
        } else {
            self.inner.insert(start, end);
        }

        self.merge_adjacent_block();
    }

    pub fn remove(&mut self, start: usize) {
        self.inner.remove(&start);
    }

    pub fn merge_adjacent_block(&mut self) {
        let mut current = self.inner
            .keys()
            .cloned()
            .next();

        while let Some(s) = current {
            let len1 = *self.inner.get(&s).unwrap();
            let end1 = s + len1;

            if let Some((&s2, &len2)) = self.inner.range((s+1)..).next() {
                if s2 <= end1 {
                    // overlapping or adjacent blocks found
                    // update the length of the current block
                    *self.inner.get_mut(&s).unwrap() = len1 + len2 + s2 - end1;
                    // remove the next block
                    self.inner.remove(&s2);
                } else {
                    current = Some(s2);
                }
            } else {
                break;
            }
        }
    }

    pub fn to_vec(&self) -> Vec<(usize, usize)> {
        self.inner
            .to_owned()
            .into_iter()
            .collect()
    }
}