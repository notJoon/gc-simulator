#[cfg(test)]
mod alloc_tests {
    use gc_simulator::alloc::{create_free_list_heap, Allocator};

    #[test]
    fn test_find_free_block_when_space_is_sufficient() {
        let mut heap = create_free_list_heap(2, vec![(0, 4)]);
        let mut alloc = Allocator {};

        // println!("heap: {:?}", heap);
        // println!("-----------------\n");

        let result = alloc.find_free_block(&mut heap, 3);
        assert_eq!(result, Some(0));
        assert_eq!(heap.free_list.to_vec(), vec![(3, 1)]);

        // println!("heap: {:?}", heap);
    }

    #[test]
    fn test_suitable_free_block_is_first_block() {
        let mut heap = create_free_list_heap(2, vec![(2, 2), (8, 2)]);
        let mut alloc = Allocator {};

        // println!("heap: {:?}", heap);
        // println!("-----------------\n");

        let result = alloc.find_free_block(&mut heap, 2);

        assert_eq!(result, Some(2));
        assert_eq!(heap.free_list.to_vec(), vec![(8, 2)]);

        // println!("heap: {:?}", heap);
        // println!("-----------------\n");
    }

    #[test]
    fn test_find_free_block_when_space_is_limited() {
        let mut heap = create_free_list_heap(2, vec![(0, 2)]);
        let mut alloc = Allocator {};

        let result = alloc.find_free_block(&mut heap, 3);
        assert_eq!(result, None);
        assert_eq!(heap.free_list.to_vec(), vec![(0, 2)]);
    }

    #[test]
    fn test_find_free_block_with_given_alignment() {
        let mut heap = create_free_list_heap(2, vec![(1, 4)]);
        let mut alloc = Allocator {};

        let result = alloc.find_free_block(&mut heap, 3);
        assert_eq!(result, Some(1));
        assert_eq!(heap.free_list.to_vec(), vec![(1, 1)]);
    }
}