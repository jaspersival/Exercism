use std::mem::size_of;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut mut_array = array.clone();
    mut_array.sort();


    let middle_index = get_middle_index(mut_array);

    if mut_array[middle_index] > key {
        mut_array = &mut_array[..middle_index];
    }




}

fn get_middle_index(array: &[i32]) -> Option<usize>{
    if array.is_empty() {
        None
    }
    else {
        Some(array.len()/2)
    }
}

fn get_key(array: &[i32], key: i32) -> Option<usize> {
    for (idx, item) in array.iter().enumerate() {
        if *item == key {
            return Some(idx)
        }
    }
    return None
}
