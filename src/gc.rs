use std::fmt;

use crate::{controller::PIController, vm::{VirtualMachine, OpCode}, heap::{Heap, self}, object::{ObjectAddress, ObjectTrait, Object, Field, Address}};

pub struct TriColorGC {
    pub color: TriColor,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GCType {
    MarkAndSweep,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TriColor {
    White,
    Gray,
    Black,
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
                sweeped += 1;
            } else {
                freed += 1;
                vm.heap.free_list.insert(*addr, obj.size());
            }
        }

        println!("Sweeped: {}, Freed: {}", sweeped, freed);
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

pub struct MarkAndSweep {
    pub pi: PIController,
}
