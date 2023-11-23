use std::cell::RefCell;
use std::default;
use std::rc::Rc;

use crate::types::TriColor;

#[derive(Debug, PartialEq)]
pub struct Object {
    pub ident: String,
    pub reference: Vec<Rc<RefCell<Object>>>,
    pub marked: TriColor,
}

impl Object {
    pub fn new(ident: String) -> Self {
        Object {
            ident,
            ..Default::default()
        }
    }

    pub fn add_reference(&mut self, object: Object) {
        self.reference.push(Rc::new(RefCell::new(object)));
    }

    pub fn delete_reference(&mut self, object: Rc<RefCell<Object>>) {
        self.reference.retain(|x| !Rc::ptr_eq(x, &object));
    }

    pub fn to_string(&self) -> String {
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