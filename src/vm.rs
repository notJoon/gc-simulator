use core::fmt;

use crate::object::{Object, TypeValue};

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
    pub heap: Vec<Object>,
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

impl VirtualMachine {
    pub fn new(max_stack_size: usize, threshold: f64) -> Result<Self, VMError> {
        if threshold <= 0.0 || threshold >= 100.0 {
            return Err(VMError::InvalidRangeOfThreshold);
        }

        Ok(Self {
            max_stack_size,
            threshold: (max_stack_size as f64 * threshold) as usize,
            ..Default::default()
        })
    }

    pub fn push(&mut self, obj: Object) -> Result<usize, VMError> {
        if self.len() >= self.max_stack_size {
            self.op_codes.push(OpCode::Halt);
            return Err(VMError::StackOverflow);
        }

        self.stack.push(obj.to_owned());
        self.op_codes.push(OpCode::Push(obj.value.unwrap()));
        self.update_first_object();
        self.update_num_object();

        Ok(self.len())
    }

    pub fn pop(&mut self) -> Result<usize, VMError> {
        if self.is_empty() {
            self.op_codes.push(OpCode::Halt);
            return Err(VMError::StackUnderflow);
        }

        let _obj = self.stack.pop().unwrap();
        self.op_codes.push(OpCode::Pop);
        self.update_first_object();
        self.update_num_object();

        Ok(self.len())
    }

    pub fn len(&self) -> usize {
        if self.is_empty() {
            return 0;
        }

        self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn update_num_object(&mut self) -> usize {
        self.num_objects = self.heap.len();
        self.num_objects
    }

    fn update_first_object(&mut self) -> bool {
        self.first_object = self.stack.first().map(|obj| obj.to_owned());

        self.first_object.is_some()
    }
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

impl fmt::Display for VirtualMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
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
