use crate::object::Object;

#[derive(Debug, PartialEq)]
pub struct VM {
    stack: Vec<*mut Object>,
    max_stack_size: usize,
    num_objects: usize,
    first_object: Option<*mut Object>,
}