
pub fn find<T: AsRef<[U]>, U: Ord + Clone>(array: T, key: U) -> Option<usize> {
    let array = array.as_ref();
    if array.is_empty() { return None }

    let mut array_copy = array.to_vec();
    array_copy.sort();

    let idx_array_vector: Vec<(usize, U)> = array_copy.iter().enumerate().map(|(index, &ref value)| (index, value.clone())).collect();
    let mut slice = idx_array_vector.as_slice();
    let resulting_index: usize;

    while slice.len() > 1 {
        let middle_index = get_middle_index(slice);
        if slice[middle_index].1 == key {
            resulting_index = slice[middle_index].0;
            return Some(resulting_index)
        }

        if slice[middle_index].1 > key {
            slice = &slice[..middle_index];
        }
        else {
            slice = &slice[middle_index..];
        }

    }
    return if slice[0].1 == key { Some(slice[0].0) } else { None }



}

fn get_middle_index<U: Ord + Clone>(array: &[(usize, U)]) -> usize{
    array.len()/2
}

