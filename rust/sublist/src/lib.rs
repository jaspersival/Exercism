use std::cmp::Ordering;

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
                if _first_list.iter().all( |item| _second_list.iter().any(|item_second_list| item == item_second_list)){
                    Comparison::Sublist
                }
                else {Comparison::Unequal}
            }
        }
        }
    }
