use std::fmt;

use crate::{controller::PIController, vm::VirtualMachine};

pub trait GarbageCollector {
    fn collect(&self, vm: &mut VirtualMachine);
    fn ty(&self) -> GCType;
    fn new_instance(&self) -> Box<dyn GarbageCollector>;
    fn adjust_confidence(&mut self, vm: &mut VirtualMachine);
    fn trigger_gc(&mut self, vm: &mut VirtualMachine);
}

pub struct TriColorGC {
    pub color: TriColor,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GCType {
    MarkAndSweep,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TriColor {
    White,
    Gray,
    Black,
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
