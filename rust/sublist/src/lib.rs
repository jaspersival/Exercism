#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match _first_list == _second_list {
        true => Comparison::Equal,
        false => {
            if _first_list == [] { Comparison::Sublist}
            else if _second_list == [] {Comparison::Superlist}
            else {
                match _first_list.len() < _second_list.len() {
                    true => {if is_sublist(_first_list, _second_list){
                        Comparison::Sublist}
                        else {Comparison::Unequal}},
                    false => {
                            if is_sublist(_second_list, _first_list) {
                                Comparison::Superlist
                            }
                            else {Comparison::Unequal}
                        }
                    }
                }
            }
            }
        }

fn is_sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    let first_list_len = _first_list.len();
    for (index, _) in _second_list.iter().enumerate() {
        let upper_bound = index + first_list_len;
        if upper_bound > _second_list.len() {
            return false
        }
        let sublist_second_list = &_second_list[index..(upper_bound)];
        if sublist_second_list == _first_list {
            return true
            }
    }
    false
}

