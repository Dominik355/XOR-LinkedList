// Integration tests for xor-ll LinkedList<T>

use xor_ll::LinkedList;

fn make_list(values: &[i32]) -> LinkedList<i32> {
    let mut list = LinkedList::new();
    for &v in values {
        list.push_back(v);
    }
    list
}

#[test]
fn new_creates_empty_list() {
    let list: LinkedList<i32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
    assert!(list.front().is_none());
    assert!(list.back().is_none());
}

#[test]
fn is_empty_and_len_basic_behaviour() {
    let mut list = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);

    list.push_back(1);
    assert!(!list.is_empty());
    assert_eq!(list.len(), 1);

    list.pop_front();
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
}

#[test]
fn push_back_and_order() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    assert_eq!(list.len(), 3);
    assert_eq!(list.front(), Some(&1));
    assert_eq!(list.back(), Some(&3));

    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(3));
    assert!(list.pop_front().is_none());
}

#[test]
fn push_front_and_order() {
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    assert_eq!(list.len(), 3);
    assert_eq!(list.front(), Some(&3));
    assert_eq!(list.back(), Some(&1));

    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(1));
    assert!(list.pop_front().is_none());
}

#[test]
fn push_back_mut_returns_mut_ref() {
    let mut list = LinkedList::new();
    let r = list.push_back_mut(1);
    *r = 10;

    assert_eq!(list.len(), 1);
    assert_eq!(list.front(), Some(&10));
    assert_eq!(list.back(), Some(&10));
}

#[test]
fn push_front_mut_returns_mut_ref() {
    let mut list = LinkedList::new();
    let r = list.push_front_mut(1);
    *r = 10;

    assert_eq!(list.len(), 1);
    assert_eq!(list.front(), Some(&10));
    assert_eq!(list.back(), Some(&10));
}

#[test]
fn pop_front_from_empty_is_none() {
    let mut list: LinkedList<i32> = LinkedList::new();
    assert_eq!(list.pop_front(), None);
}

#[test]
fn pop_back_from_empty_is_none() {
    let mut list: LinkedList<i32> = LinkedList::new();
    assert_eq!(list.pop_back(), None);
}

#[test]
fn pop_front_and_back_single_element() {
    let mut list = LinkedList::new();
    list.push_back(1);

    assert_eq!(list.pop_front(), Some(1));
    assert!(list.is_empty());

    list.push_back(2);
    assert_eq!(list.pop_back(), Some(2));
    assert!(list.is_empty());
}

#[test]
fn front_and_back_accessors() {
    let mut list = make_list(&[1, 2, 3]);

    assert_eq!(list.front(), Some(&1));
    assert_eq!(list.back(), Some(&3));

    *list.front_mut().unwrap() = 10;
    *list.back_mut().unwrap() = 30;

    assert_eq!(list.front(), Some(&10));
    assert_eq!(list.back(), Some(&30));
}

#[test]
fn clear_empties_list() {
    let mut list = make_list(&[1, 2, 3, 4]);
    list.clear();

    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
    assert!(list.front().is_none());
    assert!(list.back().is_none());

    list.push_back(5);
    assert_eq!(list.len(), 1);
    assert_eq!(list.front(), Some(&5));
}

#[test]
fn split_off_zero_moves_all_elements() {
    let mut list = make_list(&[0, 1, 2, 3]);
    let mut right = list.split_off(0);

    assert!(list.is_empty());
    assert_eq!(right.len(), 4);
    assert_eq!(right.pop_front(), Some(0));
    assert_eq!(right.pop_front(), Some(1));
    assert_eq!(right.pop_front(), Some(2));
    assert_eq!(right.pop_front(), Some(3));
    assert!(right.pop_front().is_none());
}

#[test]
fn split_off_len_returns_empty_right() {
    let mut list = make_list(&[0, 1, 2, 3]);
    let right = list.split_off(list.len());

    assert_eq!(list.len(), 4);
    assert!(right.is_empty());
}

#[test]
fn split_off_middle_splits_correctly() {
    let mut list = make_list(&[0, 1, 2, 3, 4]);
    let mut right = list.split_off(2);

    assert_eq!(list.len(), 2);
    assert_eq!(right.len(), 3);

    assert_eq!(list.pop_front(), Some(0));
    assert_eq!(list.pop_front(), Some(1));
    assert!(list.pop_front().is_none());

    assert_eq!(right.pop_front(), Some(2));
    assert_eq!(right.pop_front(), Some(3));
    assert_eq!(right.pop_front(), Some(4));
    assert!(right.pop_front().is_none());
}

#[test]
#[should_panic]
fn split_off_panics_on_out_of_bounds() {
    let mut list = make_list(&[1, 2, 3]);
    let _ = list.split_off(4);
}

#[test]
fn mixed_push_pop_sequences() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_front(0);
    list.push_back(3);

    assert_eq!(list.len(), 4);
    assert_eq!(list.front(), Some(&0));
    assert_eq!(list.back(), Some(&3));

    assert_eq!(list.pop_front(), Some(0));
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_back(), Some(2));
    assert!(list.pop_front().is_none());
}

