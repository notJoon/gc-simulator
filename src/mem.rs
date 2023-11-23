use crate::object::Object;

#[derive(Debug, Default, PartialEq)]
pub struct Memory {
    pub objects: Vec<Object>,
}

pub trait MemoryTrait {
    fn new() -> Self;
    fn add_object(&mut self, obj: Object) -> usize;
    fn remove_object(&mut self, obj: Object) -> usize;
    fn to_string(&self) -> String;
}

impl MemoryTrait for Memory {
    fn new() -> Self {
        Default::default()
    }

    fn add_object(&mut self, obj: Object) -> usize {
        self.objects.push(obj);
        self.objects.len()
    }

    fn remove_object(&mut self, obj: Object) -> usize {
        self.objects.retain(|x| x != &obj);
        self.objects.len()
    }

    fn to_string(&self) -> String {
        format!("Memory: {:?}", self.objects)
    }
}
