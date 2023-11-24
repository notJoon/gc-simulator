use core::fmt;
use std::default;

use crate::gc::TriColor;

#[derive(Debug, PartialEq, Clone)]
pub enum TypeValue {
    Int(i32),
}

impl fmt::Display for TypeValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TypeValue::Int(ref i) => write!(f, "Int: {}", i),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    pub ident: String,
    pub value: Option<TypeValue>,
    pub reference: Vec<Object>,
    pub marked: TriColor,
}

pub trait ObjectTrait {
    fn new(ident: String, value: TypeValue) -> Self;
    fn add_reference(&mut self, object: Object) -> usize;
    fn delete_reference(&mut self, object: Object) -> usize;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn to_string(&self) -> String;
    fn get_ident(&self) -> String;
    fn get_value(&self) -> Option<TypeValue>;
}

impl ObjectTrait for Object {
    fn new(ident: String, value: TypeValue) -> Self {
        Self {
            ident,
            value: Some(value),
            ..Default::default()
        }
    }

    fn add_reference(&mut self, object: Object) -> usize {
        self.reference.push(object);
        self.reference.len()
    }

    fn delete_reference(&mut self, object: Object) -> usize {
        self.reference.retain(|x| x != &object);
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

    fn get_ident(&self) -> String {
        self.ident.clone()
    }

    fn get_value(&self) -> Option<TypeValue> {
        self.value.clone()
    }
}

impl default::Default for Object {
    fn default() -> Self {
        Object {
            ident: String::from(""),
            value: None,
            reference: Vec::new(),
            marked: TriColor::White,
        }
    }
}
