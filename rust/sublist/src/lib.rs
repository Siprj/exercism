#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    use Comparison::*;
    match (first_list.len(), second_list.len()) {
        (0, 0) => Equal,
        (_, 0) => Superlist,
        (0, _) => Sublist,
        (l1, l2) if l1 > l2 => {
            if first_list
                .windows(second_list.len())
                .any(|v| v == second_list)
            {
                Superlist
            } else {
                Unequal
            }
        }
        (l1, l2) if l1 < l2 => {
            if second_list
                .windows(first_list.len())
                .any(|v| v == first_list)
            {
                Sublist
            } else {
                Unequal
            }
        }
        (_, _) => {
            if first_list == second_list {
                Equal
            } else {
                Unequal
            }
        }
    }
}
