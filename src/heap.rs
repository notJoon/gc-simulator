use std::collections::{BTreeSet, BTreeMap};

use crate::{object::{ObjectAddress, Object}, free_list::FreeList, mem::Memory};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Heap {
    pub roots: BTreeSet<ObjectAddress>,
    pub objects: BTreeMap<ObjectAddress, Object>,
    pub free_list: FreeList,
    pub memory: Vec<Memory>,
    pub alignment: usize,
}