#[cfg(test)]
mod tests {
    use gc_simulator::mem::{Memory, MemoryTrait};
    use gc_simulator::object::{Object, ObjectTrait, TypeValue};

    #[test]
    fn test_new_memory() {
        let memory = Memory::new();
        assert_eq!(memory.objects, vec![]);
    }

    #[test]
    fn test_add_object() {
        let mut memory = Memory::new();
        let value = TypeValue::Int(1);
        let object = Object::new(String::from("test"), value);
        memory.add_object(object);
        assert_eq!(memory.objects.len(), 1);
        assert_eq!(memory.objects[0].ident, "test");
        assert_eq!(memory.objects[0].value, Some(TypeValue::Int(1)));

        memory.add_object(Object::new(String::from("test2"), TypeValue::Int(2)));
        assert_eq!(memory.objects.len(), 2);
        assert_eq!(memory.objects[1].ident, "test2");
        assert_eq!(memory.objects[1].value, Some(TypeValue::Int(2)));
    }

    #[test]
    fn test_remove_object() {
        let mut memory = Memory::new();
        let value = TypeValue::Int(1);
        let object = Object::new(String::from("test"), value);
        memory.add_object(object);
        assert_eq!(memory.objects.len(), 1);

        assert_eq!(memory.objects[0].ident, "test");
        assert_eq!(memory.objects[0].value, Some(TypeValue::Int(1)));

        memory.remove_object(memory.objects[0].clone());
        assert_eq!(memory.objects.len(), 0);
    }

    #[test]
    fn test_to_string() {
        let mut memory = Memory::new();
        let value = TypeValue::Int(42);
        let object = Object::new(String::from("test"), value);
        memory.add_object(object);
        assert_eq!(
            memory.to_string(),
            "Memory: [Object { ident: \"test\", value: Some(Int(42)), reference: [], marked: White }]"
        );
    }
}
