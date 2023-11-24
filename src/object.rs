use core::fmt;
use std::{default, collections::HashSet};

use rand::Rng;

use crate::gc::TriColor;

pub type ObjectAddress = usize;

#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    /// The name of the object
    pub ident: String,
    /// The value of the object
    pub value: Option<TypeValue>,
    /// The header of the object. Which contains the size, next, and marked or not.
    pub header: ObjectHeader,
    /// The address of the object. address is a UUID.
    pub addr: ObjectAddress,
    /// The references of the object. Which contains the address of the object.
    pub references: HashSet<ObjectAddress>,
    /// The fields of the object. Which contains the value of the object.
    /// Object's size is the size of the fields.
    pub fields: Vec<Field>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectHeader {
    pub size: usize,
    pub next: Option<Address>,
    pub marked: TriColor,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Address {
    Ptr(ObjectAddress),
    NullPtr,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Field {
    Value(TypeValue),
    Ref(Address),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeValue {
    Int(i32),
}

pub trait ObjectTrait {
    fn new(ident: String, value: TypeValue) -> Self;
    fn len(&self) -> usize;
    fn add_reference(&mut self, obj: Object) -> usize;
    fn remove_reference(&mut self, obj: Object) -> usize;
    fn get_address(&self) -> ObjectAddress;
    fn is_empty(&self) -> bool;
    fn get_ident(&self) -> String;
    fn get_value(&self) -> Option<TypeValue>;
    fn is_marked(&self) -> bool;
    fn object_size(&self) -> usize;
    fn create_random_object() -> Self;
    fn to_string(&self) -> String;
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

    fn get_address(&self) -> ObjectAddress {
        self.addr
    }

    fn add_reference(&mut self, obj: Object) -> usize {
        self.references.insert(obj.addr);
        self.references.len()
    }

    fn remove_reference(&mut self, obj: Object) -> usize {
        self.references.remove(&obj.addr);
        self.references.len()
    }

    fn len(&self) -> usize {
        self.header.size
    }

    fn object_size(&self) -> usize {
        (self.fields.len() * 8) + self.header.size
    }

    fn is_empty(&self) -> bool {
        self.header.size == 0
    }

    fn is_marked(&self) -> bool {
        self.header.marked != TriColor::White
    }

    fn create_random_object() -> Self {
        let mut rng = rand::thread_rng();

        let num_fields = rng.gen_range(0..10);

        let fields: Vec<Field> = (0..num_fields).map(|_| {
            match rng.gen_range(0..=1) {
                0 => Field::Ref(Address::NullPtr),
                _ => Field::Value(TypeValue::Int(rng.gen_range(0..100)))
            }
        }).collect();

        Self {
            ident: String::from("Random Object"),
            fields,
            ..Default::default()
        }
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("Object: {}\n", self.ident));
        s.push_str(&format!("Address: {:?}\n", self.addr));
        s.push_str(&format!("Size: {} bytes\n", self.object_size()));
        s.push_str(&format!("Marked: {:?}\n", self.header.marked));
        s.push_str(&format!("References: {:?}\n", self.references));
        s.push_str(&format!("Fields: {:?}\n", self.fields));
        s
    }
}

impl Field {
    pub fn new_instance(value: TypeValue) -> Self {
        match value {
            TypeValue::Int(i) => Field::Value(TypeValue::Int(i)),
        }
    }

    pub fn new_ref(addr: ObjectAddress) -> Self {
        Field::Ref(Address::Ptr(addr))
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
            addr: uuid::Uuid::new_v4().as_u128() as usize,
            references: HashSet::new(),
            fields: vec![],
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