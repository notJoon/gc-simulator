#[cfg(test)]
mod tests {
    use gc_simulator::mem::{Memory, MemoryTrait};
    use gc_simulator::object::{Object, ObjectTrait};

    #[test]
    fn test_new_memory() {
        let memory = Memory::new();
        assert_eq!(memory.objects, vec![]);
    }

    #[test]
    fn test_add_object() {
        let mut memory = Memory::new();
        let object = Object::new(String::from("test"));
        memory.add_object(object);
        assert_eq!(memory.objects.len(), 1);
    }

    #[test]
    fn test_remove_object() {
        let mut memory = Memory::new();
        let object = Object::new(String::from("test"));
        memory.add_object(object);
        assert_eq!(memory.objects.len(), 1);

        memory.remove_object(memory.objects[0].clone());
        assert_eq!(memory.objects.len(), 0);
    }

    #[test]
    fn test_to_string() {
        let mut memory = Memory::new();
        let object = Object::new(String::from("test"));
        memory.add_object(object);
        assert_eq!(
            memory.to_string(),
            "Memory: [Object { ident: \"test\", reference: [], marked: White }]"
        );
    }
}
