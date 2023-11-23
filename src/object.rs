use std::cell::RefCell;
use std::default;
use std::rc::Rc;

use crate::types::TriColor;

#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    pub ident: String,
    pub reference: Vec<Rc<RefCell<Object>>>,
    pub marked: TriColor,
}

pub trait ObjectTrait {
    fn new(ident: String) -> Self;
    fn add_reference(&mut self, object: Object) -> usize;
    fn delete_reference(&mut self, object: Rc<RefCell<Object>>) -> usize;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn to_string(&self) -> String;
}

impl ObjectTrait for Object {
    fn new(ident: String) -> Self {
        Self {
            ident,
            ..Default::default()
        }
    }

    fn add_reference(&mut self, object: Object) -> usize {
        self.reference.push(Rc::new(RefCell::new(object)));
        self.reference.len()
    }

    fn delete_reference(&mut self, object: Rc<RefCell<Object>>) -> usize {
        self.reference.retain(|x| !Rc::ptr_eq(x, &object));
        self.reference.len()
    }

    fn len(&self) -> usize {
        self.reference.len()
    }

    fn is_empty(&self) -> bool {
        self.reference.is_empty()
    }

    fn to_string(&self) -> String {
        format!("{}: {:?}", self.ident, self.marked)
    }
}

impl default::Default for Object {
    fn default() -> Self {
        Object {
            ident: String::from(""),
            reference: Vec::new(),
            marked: TriColor::White,
        }
    }
}
