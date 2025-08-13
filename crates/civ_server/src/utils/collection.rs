pub fn slices(items_count: usize, slice_count_target: usize) -> Vec<(usize, usize)> {
    let mut slices = vec![];
    for i in 0..slice_count_target {
        let slice_len = items_count / slice_count_target;
        let start = slice_len * i;
        let end = if i != slice_count_target - 1 {
            start + slice_len
        } else {
            items_count
        };
        slices.push((start, end))
    }
    slices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evenly_divisible_items() {
        // 10 items, 5 slices -> 2 items per slice
        let result = slices(10, 5);
        assert_eq!(result, vec![(0, 2), (2, 4), (4, 6), (6, 8), (8, 10)]);
    }

    #[test]
    fn not_evenly_divisible_items() {
        // 10 items, 3 slices -> first 2 slices: len=3, last slice gets remainder
        let result = slices(10, 3);
        assert_eq!(result, vec![(0, 3), (3, 6), (6, 10)]);
    }

    #[test]
    fn single_slice_gets_all_items() {
        // 10 items, 1 slice -> everything in one slice
        let result = slices(10, 1);
        assert_eq!(result, vec![(0, 10)]);
    }

    #[test]
    fn more_slices_than_items() {
        // 3 items, 5 slices -> many empty slices at the end
        let result = slices(3, 5);
        assert_eq!(result, vec![(0, 0), (0, 0), (0, 0), (0, 0), (0, 3)]);
    }

    #[test]
    fn zero_items_multiple_slices() {
        // 0 items, 4 slices -> all empty
        let result = slices(0, 4);
        assert_eq!(result, vec![(0, 0), (0, 0), (0, 0), (0, 0)]);
    }

    #[test]
    fn single_item_multiple_slices() {
        // 1 item, 3 slices
        let result = slices(1, 3);
        assert_eq!(result, vec![(0, 0), (0, 0), (0, 1)]);
    }
}
