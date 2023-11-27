use core::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{collections::HashSet, default};

use rand::Rng;
use uuid::Uuid;

use crate::gc::TriColor;

pub type ObjectAddress = usize;

// Global counter for generating sequential UUIDs
static COUNTER: AtomicUsize = AtomicUsize::new(0);

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

#[derive(Debug, PartialEq, Clone, Default)]
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
    fn size(&self) -> usize;
    fn create_random_object(name: Option<&str>) -> Self;
    fn inject_address(&mut self, addr: ObjectAddress);
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

    fn size(&self) -> usize {
        (self.fields.len() * 8) + self.header.size
    }

    fn is_empty(&self) -> bool {
        self.header.size == 0
    }

    fn is_marked(&self) -> bool {
        self.header.marked != TriColor::White
    }

    /// testing purpose. Create a random object.
    fn create_random_object(name: Option<&str>) -> Self {
        let ident = match name {
            Some(name) => name.to_owned(),
            None => String::from("Random Object"),
        };
        let mut rng = rand::thread_rng();

        let num_fields = rng.gen_range(0..10);

        let fields: Vec<Field> = (0..num_fields)
            .map(|_| match rng.gen_range(0..=1) {
                0 => Field::Ref(Address::NullPtr),
                _ => Field::Value(TypeValue::Int(rng.gen_range(0..100))),
            })
            .collect();

        Self {
            ident,
            fields,
            ..Default::default()
        }
    }

    // testing purpose
    fn inject_address(&mut self, addr: ObjectAddress) {
        self.addr = addr;
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
                marked: TriColor::White,
            },
            addr: generate_sequential_UUID(),
            references: HashSet::new(),
            fields: vec![],
        }
    }
}

#[allow(non_snake_case)]
/// Generates a sequential UUID for simulating memory addresses.
///
/// It is also possible to customize the address by using the `inject_address` method.
fn generate_sequential_UUID() -> usize {
    let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
    let uuid = Uuid::from_fields(
        0xa1a2a3a4,
        0xb1b2,
        0xc1b2,
        &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x42],
    );
    uuid.as_u128() as usize + counter + 3
}

impl fmt::Display for TypeValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TypeValue::Int(ref i) => write!(f, "Int: {}", i),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        s.push_str(&format!("Object: {}\n", self.ident));
        s.push_str(&format!("Address: {:?}\n", self.addr));
        s.push_str(&format!("Size: {} bytes\n", self.size()));
        s.push_str(&format!("Marked: {:?}\n", self.header.marked));
        s.push_str(&format!("References: {:?}\n", self.references));
        s.push_str(&format!("Fields: {:?}\n", self.fields));
        write!(f, "{}", s)
    }
}
