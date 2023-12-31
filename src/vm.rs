use core::fmt;

use crate::{
    gc::{GCStatus, GarbageCollector, TriColor},
    heap::Heap,
    object::{Address, Field, Object, ObjectAddress, ObjectTrait, TypeValue},
};

#[derive(Debug, PartialEq, Clone)]
pub enum VMError {
    StackOverflow,
    StackUnderflow,
    InvalidRangeOfThreshold,
    AllocationFailed,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum OpCode {
    #[default]
    Halt,
    Pop,
    Push(TypeValue),
    // push object address and size
    Mark(usize, usize),
    Sweep,
}

#[derive(Debug, PartialEq, Default, Clone)]
pub struct VirtualMachine {
    pub heap: Heap,
    pub stack: Vec<Object>,
    pub op_codes: Vec<OpCode>,
    pub max_stack_size: usize,

    pub threshold: usize,
    pub num_objects: usize,
    pub first_object: Option<Object>,

    pub gc: GarbageCollector,
}

impl VirtualMachine {
    pub fn new(
        max_stack_size: usize,
        threshold: f64,
        heap_size: usize,
        alignment: usize,
    ) -> Result<Self, VMError> {
        if threshold <= 0.0 || threshold >= 100.0 {
            return Err(VMError::InvalidRangeOfThreshold);
        }

        Ok(Self {
            heap: Heap::new(heap_size, alignment),
            max_stack_size,
            threshold: (max_stack_size as f64 * threshold) as usize,
            ..Default::default()
        })
    }

    pub fn push(&mut self, obj: Object) -> Result<usize, VMError> {
        if self.len() >= self.max_stack_size {
            self.op_codes.push(OpCode::Halt);
            return Err(VMError::StackOverflow);
        }

        self.stack.push(obj.to_owned());
        self.op_codes.push(OpCode::Push(obj.value.unwrap()));
        self.update_first_object();
        self.update_num_object();

        Ok(self.len())
    }

    pub fn pop(&mut self) -> Result<usize, VMError> {
        if self.is_empty() {
            self.op_codes.push(OpCode::Halt);
            return Err(VMError::StackUnderflow);
        }

        let _obj = self.stack.pop().unwrap();
        self.op_codes.push(OpCode::Pop);
        self.update_first_object();
        self.update_num_object();

        Ok(self.len())
    }

    pub fn len(&self) -> usize {
        if self.is_empty() {
            return 0;
        }

        self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn update_num_object(&mut self) -> usize {
        self.num_objects = self.stack.len();
        self.num_objects
    }

    fn update_first_object(&mut self) -> bool {
        self.first_object = self.stack.first().map(|obj| obj.to_owned());

        self.first_object.is_some()
    }

    pub fn init_object(&mut self) -> Option<bool> {
        self.heap.objects.iter_mut().for_each(|(_, obj)| {
            obj.header.marked = TriColor::White;
        });

        Some(true)
    }

    pub fn process_roots(&mut self) {
        for root in self.heap.roots.iter() {
            if let Some(o) = self.heap.objects.get_mut(root) {
                o.header.marked = TriColor::Gray;
            }
        }
    }

    pub fn process_object(&mut self, addr: ObjectAddress, grays: &mut Vec<ObjectAddress>) -> bool {
        if let Some(obj) = self.clone().heap.objects.get_mut(&addr) {
            match obj.header.marked {
                TriColor::Black => return true,
                TriColor::Gray => {
                    obj.header.marked = TriColor::Black;
                    self.op_codes.push(OpCode::Mark(addr, obj.size()));
                    self.push_referenced_objects(obj, grays);
                }
                TriColor::White => {
                    obj.header.marked = TriColor::Gray;
                    grays.push(addr);
                }
            }
        }

        false
    }

    fn push_referenced_objects(&mut self, object: &Object, grays: &mut Vec<ObjectAddress>) {
        for f in object.fields.iter() {
            if let Field::Ref(Address::Ptr(addr)) = f {
                match self.heap.objects.get_mut(addr) {
                    Some(ref_obj) => {
                        if ref_obj.header.marked == TriColor::White {
                            ref_obj.header.marked = TriColor::Gray;
                            grays.push(*addr);
                        }
                    }
                    None => {
                        panic!("Object not found");
                    }
                }
            }
        }
    }

    pub fn reset_heap(&mut self, size: usize) -> Option<bool> {
        self.heap = Heap::new(size, self.heap.alignment);

        Some(true)
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
            VMError::AllocationFailed => write!(f, "Allocation failed"),
        }
    }
}

impl fmt::Display for VirtualMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VM: {{\nstack: {:?},\nop_codes: {:?},\nmax_stack_size: {},\nthreshold: {},\nnum_objects: {},\nfirst_object: {:?}\n}}",
            self.stack,
            self.op_codes,
            self.max_stack_size,
            self.threshold,
            self.num_objects,
            self.first_object,
        )
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

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OpCode::Push(ref i) => write!(f, "Push: {}", i),
            OpCode::Pop => write!(f, "Pop"),
            OpCode::Halt => write!(f, "Halt"),
            OpCode::Mark(addr, color) => write!(f, "Mark: addr: {}, color: {}", addr, color),
            OpCode::Sweep => write!(f, "Sweep"),
        }
    }
}
