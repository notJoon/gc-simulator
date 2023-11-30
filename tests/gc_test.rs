#[cfg(test)]
mod marker_tests {
    use gc_simulator::{
        gc::{GarbageCollector, TriColor},
        vm::VirtualMachine, object::{Object, ObjectTrait}, heap::Heap,
    };

    fn vm_setup() -> VirtualMachine {
        VirtualMachine::new(150, 0.5, 100, 8).unwrap()
    }

    fn gc_setup() -> GarbageCollector {
        GarbageCollector::default()
    }

    #[test]
    fn test_initialization_of_objects() {
        let mut vm = vm_setup();
        let mut gc = gc_setup();

        for i in 0..10 {
            let obj = Object::create_random_object(Some(&format!("obj{}", i)));
            vm.heap.objects.insert(i, obj);
        }

        assert_eq!(
            vm.heap
                .objects
                .values()
                .all(|o| o.header.marked == TriColor::White),
            true
        );

        gc.initialize_colors(&mut vm);

        assert!(vm
            .heap
            .objects
            .values()
            .all(|o| o.header.marked == TriColor::White)
        );
    }

    #[test]
    fn test_initialize_colors() {
        let mut vm = vm_setup();
        let mut gc = gc_setup();

        let obj1 = Object::create_random_object(Some("obj1"));
        let obj2 = Object::create_random_object(Some("obj2"));

        vm.heap.objects.insert(1, obj1);
        vm.heap.objects.insert(2, obj2);

        gc.initialize_colors(&mut vm);

        for obj in vm.heap.objects.values() {
            assert_eq!(obj.header.marked, TriColor::White);
        }
    }

    #[test]
    fn test_update_color() {
        let mut vm = vm_setup();
        let gc = gc_setup();

        let obj = Object::create_random_object(Some("obj"));
        vm.heap.objects.insert(1, obj);

        gc.update_color(1, TriColor::Gray, &mut vm);

        if let Some(obj) = vm.heap.objects.get(&1) {
            assert_eq!(obj.header.marked, TriColor::Gray);
        } else {
            panic!("Object not found");
        }
    }

    #[test]
    fn test_mark_roots() {
        let mut heap = Heap::new(100, 0);
        let mut gc = gc_setup();

        // add some root addresses
        heap.roots.insert(1);
        heap.roots.insert(2);

        // add objects to the heap
        let obj1 = Object::create_random_object(Some("obj1"));
        let obj2 = Object::create_random_object(Some("obj2"));
        heap.objects.insert(1, obj1);
        heap.objects.insert(2, obj2);

        // mark roots
        gc.mark_roots(&mut heap);

        // check that roots are marked as gray color
        for addr in heap.roots.iter() {
            if let Some(obj) = heap.objects.get(addr) {
                assert_eq!(obj.header.marked, TriColor::Gray);
            } else {
                panic!("Object not found");
            }
        }
    }

    #[test]
    fn test_mark_phase() {
        let mut heap = Heap::new(100, 0);
        let gc = gc_setup();

        let mut obj1 = Object::create_random_object(Some("obj1"));
        let obj2 = Object::create_random_object(Some("obj2"));
        
        obj1.add_reference(obj2.clone());

        heap.roots.insert(1);
        heap.objects.insert(1, obj1);

        let result = gc.mark_phase(&mut heap);

        for obj in heap.objects.values() {
            assert_eq!(obj.header.marked, TriColor::Black);
        }

        assert!(result.is_some());
    }

    #[test]
    fn test_start_gc() {
        let mut heap = Heap::new(100, 0);
        let mut gc = GarbageCollector::default();

        let result = gc.start_gc(&mut heap);

        assert!(result.is_some());
    }
}
