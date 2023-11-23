use core::fmt;

use crate::{
    controller::PIController,
    object::{Object, ObjectTrait, TypeValue},
};

#[derive(Debug, PartialEq, Default)]
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

#[derive(Debug, PartialEq, Default)]
pub struct VM {
    pub stack: Vec<*mut Object>,
    pub max_stack_size: usize,

    pub threshold: usize,
    pub num_objects: usize,
    pub first_object: Option<*mut Object>,

    pi: PIController,
    pub gc_confidence: f64,
    pub trigger_gc: bool,
    pub gc_status: GCStatus,
}

pub trait VMTrait {
    fn new(max_stack_size: usize, threshold: f64) -> Result<Self, VMError>
    where
        Self: Sized;
    fn push(&mut self, obj: *mut Object) -> Result<usize, VMError>;
    fn pop(&mut self) -> Result<*mut Object, VMError>;
    fn new_object(&mut self, ident: String, value: TypeValue) -> *mut Object;
    fn push_int(&mut self, value: i32) -> Result<i32, VMError>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn update_gc_confidence(&mut self) -> f64;
}

impl VMTrait for VM {
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

    fn push(&mut self, obj: *mut Object) -> Result<usize, VMError> {
        if self.len() >= self.max_stack_size {
            return Err(VMError::StackOverflow);
        }

        self.stack.push(obj);
        Ok(self.len())
    }

    fn pop(&mut self) -> Result<*mut Object, VMError> {
        if self.is_empty() {
            return Err(VMError::StackUnderflow);
        }

        let obj = self.stack.pop().unwrap();
        Ok(obj)
    }

    fn new_object(&mut self, ident: String, value: TypeValue) -> *mut Object {
        let obj = Box::new(Object::new(ident, value));
        let obj_ptr = Box::into_raw(obj);

        self.first_object = Some(obj_ptr);
        self.num_objects += 1;

        obj_ptr
    }

    fn push_int(&mut self, value: i32) -> Result<i32, VMError> {
        let obj = self.new_object(String::from("int"), TypeValue::Int(value));
        self.push(obj)?;
        Ok(value)
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn update_gc_confidence(&mut self) -> f64 {
        let current_metric = self.stack.len() as f64 / self.max_stack_size as f64 * 100.0;
        let set_point = self.threshold as f64;

        if let Ok(output) = self.pi.update(current_metric, set_point, 0.0, 0.0) {
            self.gc_confidence = output;
        }

        self.gc_confidence
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

impl fmt::Display for GCStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GCStatus::Idle => write!(f, "Idle"),
            GCStatus::Marking => write!(f, "Marking"),
            GCStatus::Sweeping => write!(f, "Sweeping"),
        }
    }
}
