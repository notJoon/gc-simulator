use core::fmt;
use std::{default, collections::HashSet};

use crate::gc::TriColor;

pub type ObjectAddress = usize;

#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    pub ident: String,
    pub value: Option<TypeValue>,
    // pub references: HashSet<ObjectAddress>,
    pub header: ObjectHeader,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectHeader {
    pub size: usize,
    pub next: Option<Address>,
    pub marked: TriColor,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Address {
    pub addr: ObjectAddress,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeValue {
    Int(i32),
}

pub trait ObjectTrait {
    fn new(ident: String, value: TypeValue) -> Self;
    fn len(&self) -> usize;
    // fn add_reference(&mut self, obj: Object) -> usize;
    // fn remove_reference(&mut self, obj: Object) -> usize;
    // fn get_addresses(&self) -> ObjectAddress;
    fn is_empty(&self) -> bool;
    fn get_ident(&self) -> String;
    fn get_value(&self) -> Option<TypeValue>;
    // fn to_string(&self) -> String;
}

impl ObjectTrait for Object {
    fn new(ident: String, value: TypeValue) -> Self {
        Self {
            ident,
            value: Some(value),
            ..Default::default()
        }
    }

    fn get_ident(&self) -> String {
        self.ident.clone()
    }

    fn get_value(&self) -> Option<TypeValue> {
        self.value.clone()
    }

    fn len(&self) -> usize {
        self.header.size
    }

    fn is_empty(&self) -> bool {
        self.header.size == 0
    }
}

impl default::Default for Object {
    fn default() -> Self {
        Object {
            ident: String::from(""),
            value: None,
            header: ObjectHeader { 
                size: std::mem::size_of::<Self>(),
                next: None,
                marked: TriColor::White
            },
        }
    }
}

impl fmt::Display for TypeValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TypeValue::Int(ref i) => write!(f, "Int: {}", i),
        }
    }
}