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
