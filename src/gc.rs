use std::fmt;

use crate::{
    controller::{PIConfig, PIController},
    object::{ObjectAddress, ObjectTrait},
    vm::{OpCode, VirtualMachine},
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
}

pub trait Marker {
    fn mark_from_roots(&self, vm: &mut VirtualMachine) -> Option<bool> {
        vm.init_object();
        vm.process_roots();

        Some(true)
    }

    fn mark(&self, addr: ObjectAddress, vm: &mut VirtualMachine) {
        let mut grays = vec![];
        grays.push(addr);

        while let Some(curr_addr) = grays.pop() {
            if vm.process_object(curr_addr, &mut grays) {
                vm.op_codes.push(OpCode::Mark(
                    curr_addr,
                    vm.heap.objects.get(&curr_addr).unwrap().size(),
                ));
                continue;
            }
        }
    }

    fn sweep(&self, vm: &mut VirtualMachine) {
        let mut sweeped = 0;
        let mut freed = 0;

        for (addr, obj) in vm.heap.objects.iter_mut() {
            if obj.header.marked == TriColor::Black {
                obj.header.marked = TriColor::White;
                vm.op_codes.push(OpCode::Sweep);
                sweeped += 1;
            } else {
                freed += 1;
                vm.heap.free_list.insert(*addr, obj.size());
            }
        }

        println!("Sweeped: {}, Freed: {}", sweeped, freed);
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
