#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn include<T: PartialEq>(small: &[T], big: &[T]) -> bool {
    for x in 0..=(big.len() - small.len()) {
        let slice = &big[x..(x + small.len())];
        if small == slice {
            return true;
        }
    }
    false
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if first_list.len() < second_list.len() && include(first_list, second_list) {
        Comparison::Sublist
    } else if first_list.len() > second_list.len() && include(second_list, first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
