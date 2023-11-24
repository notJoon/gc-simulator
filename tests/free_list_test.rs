#[cfg(test)]
mod free_list_tests {
    use gc_simulator::free_list::FreeList;

    #[test]
    fn test_no_merge() {
        let mut fl = FreeList::new(vec![(1, 3), (5, 7)]);
        fl.merge_adjacent_block();

        assert_eq!(fl.to_vec(), vec![(1, 3), (5, 7)]);
    }

    #[test]
    fn test_range_start_and_end_are_same() {
        let mut fl = FreeList::new(vec![(1, 1)]);
        fl.merge_adjacent_block();

        assert_eq!(fl.to_vec(), vec![(1, 1)]);
    }

    #[test]
    fn test_merge_adjacent_block_range() {
        let mut fl = FreeList::new(vec![(1, 3), (4, 7), (8, 10)]);
        fl.merge_adjacent_block();

        assert_eq!(fl.to_vec(), vec![(1, 17)]);
    }

    #[test]
    fn test_merge_overlapping_range() {
        let mut fl = FreeList::new(vec![(1, 5), (3, 7)]);
        fl.merge_adjacent_block();

        assert_eq!(fl.to_vec(), vec![(1, 9)]);
    }

    #[test]
    fn merge_multiple_ranges_two_left() {
        let mut fl = FreeList::new(vec![(1, 3), (4, 7), (12, 5), (17, 5)]);
        fl.merge_adjacent_block();

        assert_eq!(fl.to_vec(), vec![(1, 10), (12, 10)]);
    }

    #[test]
    fn test_merge_multiple_ranges_collapse_all() {
        let mut fl = FreeList::new(vec![(1, 3), (4, 7), (10, 13), (12, 15)]);
        fl.merge_adjacent_block();

        assert_eq!(fl.to_vec(), vec![(1, 26)]);
    }

    #[test]
    fn test_merge_multiple_small_blocks() {
        let mut fl = FreeList::new(vec![(1, 1), (2, 1), (3, 1), (5, 1), (6, 1)]);
        fl.merge_adjacent_block();

        assert_eq!(fl.to_vec(), vec![(1, 3), (5, 2)]);
    }

    #[test]
    fn test_remove_middle() {
        let mut fl = FreeList::new(vec![(1, 5), (6, 5), (11, 5)]);
        fl.remove(6);

        assert_eq!(fl.to_vec(), vec![(1, 5), (11, 5)]);
    }

    #[test]
    fn test_insert_block_in_the_middle() {
        let mut fl = FreeList::new(vec![(1, 5), (11, 5)]);
        fl.insert(6, 5);

        assert_eq!(fl.to_vec(), vec![(1, 15)]);
    }

    #[test]
    fn test_too_many_inputs() {
        let blocks: Vec<(usize, usize)> = (0..2000).map(|x| (x, 1)).collect();
        let mut fl = FreeList::new(blocks);
        fl.merge_adjacent_block();

        assert_eq!(fl.to_vec(), vec![(0, 2000)]);
    }

    #[test]
    fn test_insert() {
        let mut fl = FreeList::new(vec![(8, 8)]);
        
        fl.insert(8, 4);
        assert_eq!(fl.inner.len(), 1);
        assert_eq!(fl.inner.get(&8), Some(&8));

        fl.insert(8, 16);
        assert_eq!(fl.inner.len(), 1);
        assert_eq!(fl.inner.get(&8), Some(&16));

        fl.insert(24, 2);
        assert_eq!(fl.inner.len(), 1);
        assert_eq!(fl.inner.get(&8), Some(&18));

        fl.insert(28, 2);
        assert_eq!(fl.inner.len(), 2);
        assert_eq!(fl.inner.get(&8), Some(&18));
        assert_eq!(fl.inner.get(&28), Some(&2));
    }
}
