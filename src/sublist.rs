use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match first_list.len().cmp(&second_list.len()) {
        Ordering::Less => {
            if contains(second_list, first_list) {
                return Comparison::Sublist;
            }
        }
        Ordering::Equal => {
            if first_list == second_list {
                return Comparison::Equal;
            }
        }
        Ordering::Greater => {
            if contains(first_list, second_list) {
                return Comparison::Superlist;
            }
        }
    }

    Comparison::Unequal
}

fn contains<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    b.is_empty() || a.windows(b.len()).any(|candidate| candidate == b)
}

#[cfg(test)]
mod tests {
    use crate::sublist::{sublist, Comparison};

    #[test]
    fn empty_equals_empty() {
        let v: &[u32] = &[];

        assert_eq!(Comparison::Equal, sublist(v, v));
    }

    #[test]
    fn test_empty_is_a_sublist_of_anything() {
        assert_eq!(Comparison::Sublist, sublist(&[], &['a', 's', 'd', 'f']));
    }

    #[test]
    fn test_anything_is_a_superlist_of_empty() {
        assert_eq!(Comparison::Superlist, sublist(&['a', 's', 'd', 'f'], &[]));
    }

    #[test]
    fn test_1_is_not_2() {
        assert_eq!(Comparison::Unequal, sublist(&[1], &[2]));
    }

    #[test]
    fn test_compare_larger_equal_lists() {
        use std::iter::repeat;

        let v: Vec<char> = repeat('x').take(1000).collect();

        assert_eq!(Comparison::Equal, sublist(&v, &v));
    }

    #[test]
    fn test_sublist_at_start() {
        assert_eq!(Comparison::Sublist, sublist(&[1, 2, 3], &[1, 2, 3, 4, 5]));
    }

    #[test]
    fn sublist_in_middle() {
        assert_eq!(Comparison::Sublist, sublist(&[4, 3, 2], &[5, 4, 3, 2, 1]));
    }

    #[test]
    fn sublist_at_end() {
        assert_eq!(Comparison::Sublist, sublist(&[3, 4, 5], &[1, 2, 3, 4, 5]));
    }

    #[test]
    fn partially_matching_sublist_at_start() {
        assert_eq!(Comparison::Sublist, sublist(&[1, 1, 2], &[1, 1, 1, 2]));
    }

    #[test]
    fn sublist_early_in_huge_list() {
        let huge: Vec<u32> = (1..1_000_000).collect();

        assert_eq!(Comparison::Sublist, sublist(&[3, 4, 5], &huge));
    }

    #[test]
    fn huge_sublist_not_in_huge_list() {
        let v1: Vec<u64> = (10..1_000_001).collect();

        let v2: Vec<u64> = (1..1_000_000).collect();

        assert_eq!(Comparison::Unequal, sublist(&v1, &v2));
    }

    #[test]
    fn superlist_at_start() {
        assert_eq!(Comparison::Superlist, sublist(&[1, 2, 3, 4, 5], &[1, 2, 3]));
    }

    #[test]
    fn superlist_in_middle() {
        assert_eq!(Comparison::Superlist, sublist(&[5, 4, 3, 2, 1], &[4, 3, 2]));
    }

    #[test]
    fn superlist_at_end() {
        assert_eq!(Comparison::Superlist, sublist(&[1, 2, 3, 4, 5], &[3, 4, 5]));
    }

    #[test]
    fn second_list_missing_element_from_first_list() {
        assert_eq!(Comparison::Unequal, sublist(&[1, 2, 3], &[1, 3]));
    }

    #[test]
    fn superlist_early_in_huge_list() {
        let huge: Vec<u32> = (1..1_000_000).collect();

        assert_eq!(Comparison::Superlist, sublist(&huge, &[3, 4, 5]));
    }

    #[test]
    fn recurring_values_sublist() {
        assert_eq!(
            Comparison::Sublist,
            sublist(&[1, 2, 1, 2, 3], &[1, 2, 3, 1, 2, 1, 2, 3, 2, 1])
        );
    }

    #[test]
    fn recurring_values_unequal() {
        assert_eq!(
            Comparison::Unequal,
            sublist(&[1, 2, 1, 2, 3], &[1, 2, 3, 1, 2, 3, 2, 3, 2, 1])
        );
    }
}
