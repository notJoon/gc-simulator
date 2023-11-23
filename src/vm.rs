use core::fmt;

use crate::object::{Object, ObjectTrait, TypeValue};

#[derive(Debug, PartialEq, Default)]
pub struct VM {
    pub stack: Vec<*mut Object>,
    pub max_stack_size: usize,
    pub threshold: usize,
    pub num_objects: usize,
    pub first_object: Option<*mut Object>,
    pub gc_trigger: bool,
}

pub trait VMTrait {
    fn new(max_stack_size: usize) -> Self;
    fn push(&mut self, obj: *mut Object) -> Result<usize, VMError>;
    fn pop(&mut self) -> Result<*mut Object, VMError>;
    fn new_object(&mut self, ident: String, value: TypeValue) -> *mut Object;
    fn push_int(&mut self, value: i32) -> Result<i32, VMError>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn gc_trigger(&mut self) -> bool;
}

impl VMTrait for VM {
    fn new(max_stack_size: usize) -> Self {
        Self {
            max_stack_size,
            ..Default::default()
        }
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

    fn gc_trigger(&mut self) -> bool {
        // use PI as a trigger for GC
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum VMError {
    StackOverflow,
    StackUnderflow,
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VMError::StackOverflow => write!(f, "Stack Overflow"),
            VMError::StackUnderflow => write!(f, "Stack Underflow"),
        }
    }
}
