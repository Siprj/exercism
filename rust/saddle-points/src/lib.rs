pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.is_empty() {
        return vec![];
    }

    let mut ret = vec![];
    let mut max_rows = vec![];
    let mut mix_column: Vec<u64> = (0..input[0].len()).map(|_| u64::MAX).collect();
    for y in 0..input.len() {
        let mut row_max = u64::MIN;
        for x in 0..input[0].len() {
            mix_column[x] = mix_column[x].min(input[y][x]);
            row_max = row_max.max(input[y][x]);
        }
        max_rows.push(row_max);
    }
    for y in 0..max_rows.len() {
        for x in 0..mix_column.len() {
            if max_rows[y] == mix_column[x] {
                ret.push((y, x));
            }
        }
    }
    ret
}
