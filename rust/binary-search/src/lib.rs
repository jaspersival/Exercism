
pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() { return None }

    let mut array_copy = array.to_vec();
    array_copy.sort();

    let idx_array_vector: Vec<Vec<i32>> = array_copy.iter().enumerate().map(|(index, &value)| vec![index as i32, value]).collect();
    let mut slice = idx_array_vector.as_slice();
    let resulting_index: i32;

    while slice.len() > 1 {
        let middle_index = get_middle_index(slice);
        if slice[middle_index][1] == key {
            resulting_index = slice[middle_index][0];
            return Some(resulting_index as usize)
        }

        if slice[middle_index][1] > key {
            slice = &slice[..middle_index];
        }
        else {
            slice = &slice[middle_index..];
        }

    }
    return if slice[0][1] == key { Some(slice[0][0] as usize) } else { None }



}

fn get_middle_index(array: &[Vec<i32>]) -> usize{
    array.len()/2
}

