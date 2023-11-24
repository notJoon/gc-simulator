use core::fmt;

use crate::{
    gc::TriColor,
    object::{Object, TypeValue},
};

#[derive(Debug, PartialEq, Default, Clone)]
pub enum GCStatus {
    #[default]
    Idle,
    Marking,
    Sweeping,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VMError {
    StackOverflow,
    StackUnderflow,
    InvalidRangeOfThreshold,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum OpCode {
    #[default]
    Halt,
    Pop,
    Push(TypeValue),
    Mark,
    Sweep,
}

#[derive(Debug, PartialEq, Default, Clone)]
pub struct VirtualMachine {
    pub stack: Vec<Object>,
    pub op_codes: Vec<OpCode>,
    pub max_stack_size: usize,

    pub threshold: usize,
    pub num_objects: usize,
    pub first_object: Option<Object>,

    pub gc_confidence: f64,
    pub trigger_gc: bool,
    pub gc_status: GCStatus,
}

pub trait VMTrait {
    fn new(max_stack_size: usize, threshold: f64) -> Result<Self, VMError>
    where
        Self: Sized;
    fn push(&mut self, obj: Object) -> Result<usize, VMError>;
    fn pop(&mut self) -> Result<Object, VMError>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn to_string(&self) -> String;

    // fn mark(&mut self);
}

impl VMTrait for VirtualMachine {
    fn new(max_stack_size: usize, threshold: f64) -> Result<Self, VMError> {
        if threshold <= 0.0 || threshold >= 100.0 {
            return Err(VMError::InvalidRangeOfThreshold);
        }

        Ok(Self {
            max_stack_size,
            threshold: (max_stack_size as f64 * threshold) as usize,
            ..Default::default()
        })
    }

    fn push(&mut self, obj: Object) -> Result<usize, VMError> {
        if self.len() >= self.max_stack_size {
            self.op_codes.push(OpCode::Halt);
            return Err(VMError::StackOverflow);
        }

        self.stack.push(obj.to_owned());
        self.op_codes.push(OpCode::Push(obj.value.unwrap()));
        self.first_object = Some(self.stack[0].to_owned());

        Ok(self.len())
    }

    fn pop(&mut self) -> Result<Object, VMError> {
        if self.is_empty() {
            self.op_codes.push(OpCode::Halt);
            return Err(VMError::StackUnderflow);
        }

        let obj = self.stack.pop().unwrap();
        self.op_codes.push(OpCode::Pop);
        self.num_objects -= 1;

        Ok(obj)
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn to_string(&self) -> String {
        format!(
            "VM: {{\nstack: {:?},\nop_codes: {:?},\nmax_stack_size: {},\nthreshold: {},\nnum_objects: {},\nfirst_object: {:?},\ngc_confidence: {},\ntrigger_gc: {},\ngc_status: {}\n}}",
            self.stack,
            self.op_codes,
            self.max_stack_size,
            self.threshold,
            self.num_objects,
            self.first_object,
            self.gc_confidence,
            self.trigger_gc,
            self.gc_status
        )
    }

    // fn mark(&mut self) {
    //     self.op_codes.push(OpCode::Mark);
    //     let mut mark_stack = Vec::new();

    //     while let Some(mut obj) = self.stack.pop() {
    //         if obj.marked == TriColor::White {
    //             obj.marked = TriColor::Gray;
    //             mark_stack.push(obj.clone());
    //         }

    //         if obj.reference.len() > 0 {
    //             let mut ref_obj = obj.reference.pop().unwrap();
    //             if ref_obj.marked == TriColor::White {
    //                 ref_obj.marked = TriColor::Gray;
    //                 mark_stack.push(ref_obj.clone());
    //             }
    //             obj.reference.push(ref_obj);
    //         }
    //     }

    //     while let Some(mut obj) = mark_stack.pop() {
    //         if obj.marked == TriColor::Gray {
    //             obj.marked = TriColor::Black;
    //         }

    //         if obj.reference.len() > 0 {
    //             let mut ref_obj = obj.reference.pop().unwrap();
    //             if ref_obj.marked == TriColor::Gray {
    //                 ref_obj.marked = TriColor::Black;
    //             }
    //             obj.reference.push(ref_obj);
    //         }
    //     }
    // }
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VMError::StackOverflow => write!(f, "Stack Overflow"),
            VMError::StackUnderflow => write!(f, "Stack Underflow"),
            VMError::InvalidRangeOfThreshold => write!(
                f,
                "Invalid range of threshold. Must be between 0.0 and 100.0"
            ),
        }
    }
}

impl fmt::Display for GCStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GCStatus::Idle => write!(f, "Idle"),
            GCStatus::Marking => write!(f, "Marking"),
            GCStatus::Sweeping => write!(f, "Sweeping"),
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OpCode::Push(ref i) => write!(f, "Push: {}", i),
            OpCode::Pop => write!(f, "Pop"),
            OpCode::Halt => write!(f, "Halt"),
            OpCode::Mark => write!(f, "Mark"),
            OpCode::Sweep => write!(f, "Sweep"),
        }
    }
}
