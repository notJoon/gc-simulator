use std::{collections::BTreeSet, fmt};

use crate::{
    controller::{PIConfig, PIController},
    heap::Heap,
    object::{ObjectAddress, ObjectTrait},
    vm::VirtualMachine,
};

pub struct TriColorGC {
    pub color: TriColor,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum GCType {
    #[default]
    MarkAndSweep,
}

#[derive(Debug, PartialEq, Default, Clone)]
pub enum GCStatus {
    #[default]
    Idle,
    Marking,
    Sweeping,
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum TriColor {
    #[default]
    White,
    Gray,
    Black,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GarbageCollector {
    pub gc_typ: GCType,
    pub gc_status: GCStatus,
    pub confidence: f64,
    pub pi_controller: PIController,
}

impl GarbageCollector {
    pub fn initialize_colors(&mut self, vm: &mut VirtualMachine) {
        vm.heap.objects.values_mut().for_each(|o| {
            o.header.marked = TriColor::White;
        })
    }

    pub fn update_color(&self, addr: ObjectAddress, new_color: TriColor, vm: &mut VirtualMachine) {
        if let Some(obj) = vm.heap.objects.get_mut(&addr) {
            obj.header.marked = new_color;
        }
    }

    pub fn start_gc(&mut self, heap: &mut Heap) -> Option<bool> {
        for object in heap.objects.values_mut() {
            object.header.marked = TriColor::White;
        }

        self.mark_roots(heap);
        self.mark_phase(heap);

        Some(true)
    }

    /// mark_roots mark root object as gray
    pub fn mark_roots(&mut self, heap: &mut Heap) -> bool {
        for addr in heap.roots.iter() {
            if let Some(obj) = heap.objects.get_mut(addr) {
                obj.header.marked = TriColor::Gray;
            }
        }

        true
    }

    // Steps to Implement Mark phase:
    // 1. Iterate Over Gray Objects: Continue processing objects as long as there are
    //   gray objects in the list (here: `grays`).
    //
    // 2. Mark Objects Black: When an object is processed, mark it black to indicate
    //   that it has been visited.
    //
    // 3. Mark Referenced Objects Gray: For each object processed, examine its fields
    //   for references to other objects. It a reference object is *White*, mark it gray and add it to the
    //   list of gray objects to be processed.
    //
    // 4. Handle Circular References: Avoid re-adding black objects to the list of gray objects.
    pub fn mark_phase(&self, heap: &mut Heap) -> Option<BTreeSet<usize>> {
        let mut grays = heap.roots.clone();

        while let Some(addr) = grays.iter().next().cloned() {
            if let Some(obj) = heap.objects.get_mut(&addr) {
                obj.header.marked = TriColor::Black;

                for ref_addr in obj.get_references() {
                    if let Some(ref_obj) = heap.objects.get_mut(&ref_addr) {
                        if ref_obj.header.marked == TriColor::White {
                            ref_obj.header.marked = TriColor::Gray;
                            grays.insert(ref_addr);
                        }
                    }
                }
            }
        }

        Some(grays)
    }
}

impl Default for GarbageCollector {
    fn default() -> Self {
        Self {
            gc_typ: GCType::MarkAndSweep,
            gc_status: GCStatus::Idle,
            confidence: 0.0,
            pi_controller: PIController::new(PIConfig::default()),
        }
    }
}

impl fmt::Display for TriColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TriColor::White => write!(f, "White: Not visited"),
            TriColor::Gray => write!(f, "Gray: Visiting"),
            TriColor::Black => write!(f, "Black: Visited"),
        }
    }
}
