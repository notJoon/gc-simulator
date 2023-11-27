#[cfg(test)]
mod marker_tests {
    use gc_simulator::{vm::VirtualMachine, gc::{GarbageCollector, TriColor}, object::{Object, ObjectTrait, TypeValue}};

    fn vm_setup() -> VirtualMachine {
        VirtualMachine::new(10, 0.5, 100, 8).unwrap()
    }

    fn gc_setup() -> GarbageCollector {
        GarbageCollector::default()
    }

    #[test]
    fn test_initialization_of_objects() {
        let mut vm = vm_setup();
        let mut gc = gc_setup();

        assert_eq!(vm.heap.objects.values().all(|o| o.header.marked == TriColor::White), true);

        gc.initialize_colors(&mut vm);

        assert!(vm.heap.objects.values().all(|o| o.header.marked == TriColor::White));
    }

    #[test]
    fn test_update_color() {
        let mut vm = vm_setup();
        let gc = gc_setup();

        let obj1 = Object::new("obj1".to_string(), TypeValue::Int(42));
        let obj2 = Object::new("obj2".to_string(), TypeValue::Int(42));

        vm.push(obj1).unwrap();
        vm.push(obj2).unwrap();

        let obj1_addr = vm.stack[0].get_address();
        gc.update_color(obj1_addr, TriColor::Gray, &mut vm);

        assert_eq!(vm.stack[0].header.marked, TriColor::Gray);
    }
}
