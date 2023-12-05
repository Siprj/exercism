pub fn find<T, K>(array: T, key: K) -> Option<usize>
where
    T: AsRef<[K]>,
    K: Ord,
{
    let mut array = array.as_ref();
    let mut search_index = 0;
    loop {
        let middle: usize = array.len() / 2;
        if let Some(v) = array.get(array.len() / 2) {
            match key.cmp(v) {
                std::cmp::Ordering::Less => array = &array[..middle],
                std::cmp::Ordering::Equal => return Some(search_index + middle),
                std::cmp::Ordering::Greater => {
                    array = &array[middle + 1..];
                    search_index += middle + 1;
                }
            }
        } else {
            return None;
        }
    }
}
