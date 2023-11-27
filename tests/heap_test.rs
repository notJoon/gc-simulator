#[cfg(test)]
mod heap_tests {
    use gc_simulator::{
        heap::Heap,
        object::{Object, ObjectTrait}, gc::TriColor,
    };

    #[test]
    fn test_create_new_heap() {
        let heap = Heap::new(100, 0);
        assert_eq!(heap.memory.len(), 100);
        assert_eq!(heap.calculate_free_memory(), 100);
        assert_eq!(heap.objects.is_empty(), true);
    }

    #[test]
    fn test_next_object_address() {
        let mut heap = Heap::new(100, 0);

        let obj1 = Object::create_random_object(Some("o1"));
        let obj2 = Object::create_random_object(Some("o2"));
        let obj3 = Object::create_random_object(Some("o3"));

        heap.objects.insert(obj1.get_address(), obj1.to_owned());
        heap.objects.insert(obj2.get_address(), obj2.to_owned());
        heap.objects.insert(obj3.get_address(), obj3.to_owned());

        assert_eq!(heap.next_object(obj1.get_address() - 1), Some(&obj1));
        assert_eq!(heap.next_object(obj1.get_address()), Some(&obj2));
        assert_eq!(heap.next_object(obj2.get_address()), Some(&obj3));

        assert_eq!(heap.next_object(obj3.get_address()), None);
        assert_eq!(heap.next_object(obj3.get_address() + 10), None);
    }

    #[test]
    fn test_last_object_addr() {
        let mut h = Heap::new(100, 0);

        assert_eq!(h.last_object(), None);

        let obj1 = Object::create_random_object(Some("o1"));
        h.objects.insert(obj1.get_address(), obj1.to_owned());
        assert_eq!(h.last_object(), Some(&obj1));

        let obj2 = Object::create_random_object(Some("o2"));
        h.objects.insert(obj2.get_address(), obj2.to_owned());
        assert_eq!(h.last_object(), Some(&obj2));

        // println!("heap: {:?}", h);
    }

    #[test]
    fn test_next_prev_object_address() {
        let mut h = Heap::new(100, 0);

        // ad multiple objects and navigate them using next_object and prev_object
        let o1 = Object::create_random_object(Some("o1"));
        let o2 = Object::create_random_object(Some("o2"));
        let o3 = Object::create_random_object(Some("o3"));

        h.objects.insert(o1.get_address(), o1.to_owned());
        h.objects.insert(o2.get_address(), o2.to_owned());
        h.objects.insert(o3.get_address(), o3.to_owned());

        // println!("heap: {:?}", h);
        assert_eq!(h.next_object(o1.get_address()), Some(&o2));
        assert_eq!(h.next_object(o2.get_address()), Some(&o3));
        assert_eq!(h.next_object(o3.get_address()), None);

        assert_eq!(h.prev_object(o1.get_address() - 100), None);
        assert_eq!(h.next_object(o3.get_address() + 100), None);
    }

    #[test]
    fn test_calculate_free_memory() {
        let heap = Heap::new(100, 4);
        assert_eq!(heap.calculate_free_memory(), 100);
    }

    #[test]
    fn test_merge_free_ranges() {
        let mut heap = Heap::new(100, 4);
        heap.merge_free_ranges().unwrap();
        assert_eq!(heap.calculate_free_memory(), 100);
    }

    #[test]
    fn test_align_position() {
        let h = Heap::new(100, 4);

        assert_eq!(h.aligned_position(0), 0);
        assert_eq!(h.aligned_position(1), 4);
        assert_eq!(h.aligned_position(2), 4);
        assert_eq!(h.aligned_position(3), 4);
        assert_eq!(h.aligned_position(4), 4);
        assert_eq!(h.aligned_position(5), 8);

        let h = Heap::new(100, 8);

        assert_eq!(h.aligned_position(0), 0);
        assert_eq!(h.aligned_position(5), 8);
        assert_eq!(h.aligned_position(8), 8);
        assert_eq!(h.aligned_position(9), 16);
    }

    #[test]
    fn test_no_align() {
        let h = Heap::new(100, 0);

        for i in 0..10 {
            assert_eq!(h.aligned_position(i), i);
        }
    }

    #[test]
    fn test_initialization_heap_color() {
        let mut heap = Heap::new(1024, 0);
        let addr1 = heap.allocate_object(64).unwrap();
        let addr2 = heap.allocate_object(64).unwrap();

        assert_eq!(heap.objects.get(&addr1).unwrap().header.marked, TriColor::White);
        assert_eq!(heap.objects.get(&addr2).unwrap().header.marked, TriColor::White);

        heap.objects.get_mut(&addr1).unwrap().header.marked = TriColor::Gray;
        heap.objects.get_mut(&addr2).unwrap().header.marked = TriColor::Gray;

        for obj in heap.objects.values() {
            assert_eq!(obj.header.marked, TriColor::Gray);
        }

        assert_eq!(heap.objects.get(&addr1).unwrap().header.marked, TriColor::Gray);
        assert_eq!(heap.objects.get(&addr2).unwrap().header.marked, TriColor::Gray);
    }
}
