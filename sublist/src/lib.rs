#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    let mut shorter = first_list;
    let mut longer = second_list;
    let mut flag = Comparison::Sublist;

    if first_list.len() > second_list.len() {
        (shorter, longer) = (longer, shorter);
        flag = Comparison::Superlist;
    }

    if shorter.is_empty()
        || longer
            .windows(shorter.len())
            .any(|window| window == shorter)
    {
        flag
    } else {
        Comparison::Unequal
    }
}
