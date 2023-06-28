#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    use Comparison::*;
    match (_first_list.len(), _second_list.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (n, m) if n < m => {
            if _second_list.windows(n).any(|v| v == _first_list) {
                Sublist
            } else {
                Unequal
            }
        }
        (n, m) if n > m => {
            if _first_list.windows(m).any(|v| v == _second_list) {
                Superlist
            } else {
                Unequal
            }
        }
        (_, _) => {
            if _first_list == _second_list {
                Equal
            } else {
                Unequal
            }
        }
    }
}
