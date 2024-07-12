
pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() { return None }

    let mut array_copy = array.to_vec();
    array_copy.sort();
    let mut slice = array_copy.as_slice();

    while slice.len() > 1 {
        let middle_index = get_middle_index(slice);
        if slice[middle_index] == key {
            return Some(middle_index)
        }

        if slice[middle_index] > key {
            slice = &slice[..middle_index];
        }
        else {
            slice = &slice[middle_index..];
        }

    }
    return if array_copy[0] == key { Some(0) } else { None }



}

fn get_middle_index(array: &[i32]) -> usize{
    array.len()/2
}

