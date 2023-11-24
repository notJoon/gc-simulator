#[cfg(test)]
mod heap_tests {
    use gc_simulator::heap::Heap;

    #[test]
    fn test_create_new_heap() {
        let heap = Heap::new(100, 0);
        assert_eq!(heap.memory.len(), 100);
        assert_eq!(heap.calculate_free_memory(), 100);
        assert_eq!(heap.objects.is_empty(), true);
    }
}