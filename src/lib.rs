use std::cmp::Ordering;
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    value: T,
    xored: usize, // XOR of prev and next pointers
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { value, xored: 0 }
    }

    fn into_element(self: Box<Self>) -> T {
        self.value
    }
}

// implementation state of API methods from the original LinkedList
// ✔ append
// ✔ back
// ✔ back_mut
// ✔ clear
// ✔ contains
// cursor_back
// cursor_back_mut
// cursor_front
// cursor_front_mut
// extract_if
// ✔ front
// ✔ front_mut
// ✔ is_empty
// ✔ iter
// ✔ iter_mut
// ✔ len
// ✔ new
// ✔ pop_back
// ✔ pop_front
// ✔ push_back
// ✔ push_back_mut
// ✔ push_front
// ✔ push_front_mut
// remove
// retain
// ✔ split_off
#[derive(Debug)]
pub struct LinkedList<T> {
    begin: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,

    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            begin: None,
            end: None,
            len: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.begin.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_back(&mut self, value: T) {
        let _ = self.push_back_mut(value);
    }

    pub fn push_back_mut(&mut self, value: T) -> &mut T {
        let note_ptr = Box::into_raw(Box::new(Node::new(value)));
        let mut nnnode = NonNull::new(note_ptr).unwrap();
        unsafe {
            self.push_back_inner(nnnode);
            &mut nnnode.as_mut().value
        }
    }

    #[inline]
    unsafe fn push_back_inner(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            match self.end {
                None => {
                    assert!(self.begin.is_none());
                    self.begin = Some(node);
                    self.end = self.begin;
                }
                Some(end) => {
                    // could use 'as_mut()' but its just 1 more wrapping call than dereferenced .as_ptr()
                    (*node.as_ptr()).xored = end.as_ptr() as usize;
                    (*end.as_ptr()).xored ^= node.as_ptr() as usize;
                    self.end = Some(node);
                }
            }
        }
        self.len += 1;
    }

    pub fn push_front(&mut self, value: T) {
        let _ = self.push_front_mut(value);
    }

    pub fn push_front_mut(&mut self, value: T) -> &mut T {
        let note_ptr = Box::into_raw(Box::new(Node::new(value)));
        let mut nnnode = NonNull::new(note_ptr).unwrap();
        unsafe {
            self.push_front_inner(nnnode);
            &mut nnnode.as_mut().value
        }
    }

    #[inline]
    unsafe fn push_front_inner(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            match self.begin {
                None => {
                    assert!(self.end.is_none());
                    self.begin = Some(node);
                    self.end = self.begin;
                }
                Some(begin) => {
                    (*node.as_ptr()).xored = begin.as_ptr() as usize;
                    (*begin.as_ptr()).xored ^= node.as_ptr() as usize;
                    self.begin = Some(node);
                }
            }
        }
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(Node::into_element)
    }

    #[inline]
    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        match self.begin.take() {
            None => None,
            Some(begin) => unsafe {
                let next = node_next(begin, &mut 0);

                if next.is_null() {
                    // there is no next node, this is a single element list
                    assert_eq!(self.len, 1);
                    self.end = None;
                } else {
                    // remove the current beginning node from next - so it can become the new `begin`
                    (*next).xored ^= begin.as_ptr() as usize;
                    // and assign this next as the new beginning
                    self.begin = NonNull::new(next);
                }

                // decrease a length
                self.len -= 1;

                // now - new beginning is set
                // we can freely work with the original beginning as it is not part of the chain anymore\
                Some(Box::from_raw(begin.as_ptr()))
            },
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node().map(Node::into_element)
    }

    #[inline]
    fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
        match self.end.take() {
            None => None,
            Some(end) => unsafe {
                let next = node_next(end, &mut 0);

                if next.is_null() {
                    // there is no next node, this is a single element list
                    assert_eq!(self.len, 1);
                    self.begin = None;
                } else {
                    // remove the current end node from next - so it can become the new `end`
                    (*next).xored ^= end.as_ptr() as usize;
                    // and assign this next as the new end
                    self.end = NonNull::new(next);
                }

                self.len -= 1;

                Some(Box::from_raw(end.as_ptr()))
            },
        }
    }

    pub fn front(&self) -> Option<&T> {
        Self::node_ref(&self.begin)
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        Self::mut_node_ref(&mut self.begin)
    }

    pub fn back(&self) -> Option<&T> {
        Self::node_ref(&self.end)
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        Self::mut_node_ref(&mut self.end)
    }

    fn node_ref(node: &Option<NonNull<Node<T>>>) -> Option<&T> {
        node.as_ref().map(|node| unsafe { &(*node.as_ref()).value })
    }

    fn mut_node_ref(node: &mut Option<NonNull<Node<T>>>) -> Option<&mut T> {
        node.as_mut()
            .map(|node| unsafe { &mut node.as_mut().value })
    }

    pub fn clear(&mut self) {
        drop(Self {
            begin: self.begin.take(),
            end: self.end.take(),
            len: mem::take(&mut self.len),
        });
    }

    /// Splits the list into two at the given index. Returns everything after the given index,
    /// including the index.
    pub fn split_off(&mut self, at: usize) -> Self {
        assert!(at <= self.len, "Cannot split off at a nonexistent index");

        if at == 0 {
            return mem::replace(self, Self::new());
        } else if at == self.len {
            return Self::new();
        }

        let mut index = 0;
        let mut prev_ptr: usize = 0;
        let mut current = self.begin;
        while index < at {
            current = NonNull::new(node_next(current.unwrap(), &mut prev_ptr));
            index += 1;
        }

        // frst correct the original
        let at_minus_1 = prev_ptr as *mut Node<T>;
        unsafe {
            (*at_minus_1).xored ^= current.unwrap().as_ptr() as usize;
        }
        let original_end = mem::replace(&mut self.end, NonNull::new(at_minus_1));
        let original_len = mem::replace(&mut self.len, at);

        // now establish a new one
        let mut new_list = Self::new();
        unsafe {
            current = current.map(|n| {
                (*n.as_ptr()).xored ^= prev_ptr;
                n
            });
        }
        new_list.begin = current;
        new_list.end = original_end;
        new_list.len = original_len - at;

        new_list
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            begin: self.begin,
            end: self.end,
            len: self.len,
            prev: 0,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn iter_mut(&self) -> IterMut<'_, T> {
        IterMut {
            begin: self.begin,
            end: self.end,
            len: self.len,
            prev: 0,
            marker: PhantomData,
        }
    }

    pub fn contains(&self, x: &T) -> bool
    where
        T: PartialEq<T>,
    {
        self.iter().any(|e| e == x)
    }

    pub fn append(&mut self, other: &mut Self) {
        match self.end {
            None => mem::swap(self, other),
            Some(end) => {
                // take the beginning of the given other and connect it with our end
                if let Some(other_begin) = other.begin.take() {
                    unsafe {
                        (*end.as_ptr()).xored ^= other_begin.as_ptr() as usize;
                        (*other_begin.as_ptr()).xored ^= end.as_ptr() as usize;
                    }
                    self.end = other.end.take();
                    self.len += mem::replace(&mut other.len, 0);
                }
            }
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front_node().is_some() {}
    }
}

pub struct Iter<'a, T> {
    begin: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
    len: usize,
    prev: usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.begin.map(|node| unsafe {
                let return_ptr = &*node.as_ptr();
                let next = node_next(node, &mut self.prev);
                self.begin = NonNull::new(next);
                self.len -= 1;
                &return_ptr.value
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    #[inline]
    fn last(mut self) -> Option<&'a T> {
        self.next_back()
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.end.map(|node| unsafe {
                let return_ptr = &*node.as_ptr();
                let next = node_next(node, &mut self.prev);
                self.end = NonNull::new(next);
                self.len -= 1;
                &return_ptr.value
            })
        }
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {}

impl<T> FusedIterator for Iter<'_, T> {}

impl<T> Default for Iter<'_, T> {
    fn default() -> Self {
        Iter {
            begin: None,
            end: None,
            len: 0,
            prev: 0,
            marker: PhantomData,
        }
    }
}

pub struct IterMut<'a, T: 'a> {
    begin: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
    len: usize,
    prev: usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<&'a mut T> {
        if self.len == 0 {
            None
        } else {
            self.begin.map(|node| unsafe {
                let return_ptr = &mut *node.as_ptr();
                let next = node_next(node, &mut self.prev);
                self.begin = NonNull::new(next);
                self.len -= 1;
                &mut return_ptr.value
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    #[inline]
    fn last(mut self) -> Option<&'a mut T> {
        self.next_back()
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut T> {
        if self.len == 0 {
            None
        } else {
            self.end.map(|node| unsafe {
                let return_ptr = &mut *node.as_ptr();
                let next = node_next(node, &mut self.prev);
                self.end = NonNull::new(next);
                self.len -= 1;
                &mut return_ptr.value
            })
        }
    }
}

impl<T> ExactSizeIterator for IterMut<'_, T> {}

impl<T> FusedIterator for IterMut<'_, T> {}

impl<T> Default for IterMut<'_, T> {
    fn default() -> Self {
        IterMut {
            begin: None,
            end: None,
            len: 0,
            prev: 0,
            marker: PhantomData,
        }
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }

    fn ne(&self, other: &Self) -> bool {
        self.len() != other.len() || self.iter().ne(other)
    }
}

impl<T: Eq> Eq for LinkedList<T> {}

impl<T: PartialOrd> PartialOrd for LinkedList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for LinkedList<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other)
    }
}

fn node_next<T>(node: NonNull<Node<T>>, prev: &mut usize) -> *mut Node<T> {
    unsafe {
        let next = ((*node.as_ptr()).xored ^ *prev) as *mut Node<T>;
        *prev = node.as_ptr() as usize;
        next
    }
}

pub struct IntoIter<T> {
    list: LinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len, Some(self.list.len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.list.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

impl<T> FusedIterator for IntoIter<T> {}

impl<T> Default for IntoIter<T> {
    fn default() -> Self {
        LinkedList::new().into_iter()
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        iter.into_iter().for_each(|elt| list.push_back(elt));
        list
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Consumes the list into an iterator yielding elements by value.
    #[inline]
    fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_list(values: &[i32]) -> LinkedList<i32> {
        let mut list = LinkedList::new();
        for &v in values {
            list.push_back(v);
        }
        list
    }

    #[test]
    fn iter_test() {
        let list = make_list(&(5..=10).into_iter().collect::<Vec<_>>());

        assert_eq!(
            list.iter().cloned().collect::<Vec<_>>(),
            (5..=10).collect::<Vec<_>>()
        );
    }

    #[test]
    fn node_next_test() {
        let mut list = make_list(&(5..=10).into_iter().collect::<Vec<_>>());

        let mut printed = vec![];

        let mut prev: usize = 0;
        let mut iter_opt = list.end;
        while let Some(iter) = iter_opt {
            let val = unsafe { (*iter.as_ptr()).value };
            printed.push(val);
            iter_opt = NonNull::new(node_next(iter, &mut prev));
        }

        assert_eq!(printed, (5..=10).rev().collect::<Vec<_>>());
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

    #[test]
    fn iter_returns_all_elements_in_order() {
        let list = make_list(&[1, 2, 3, 4]);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut_allows_mutation() {
        let mut list = make_list(&[1, 2, 3]);
        for v in list.iter_mut() {
            *v *= 2;
        }
        let result: Vec<_> = list.iter().cloned().collect();
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn into_iterator_consumes_list_and_yields_elements() {
        let list = make_list(&[10, 20, 30]);
        let v: Vec<_> = list.into_iter().collect();
        assert_eq!(v, vec![10, 20, 30]);
    }

    #[test]
    fn append_moves_all_elements_from_other() {
        let mut a = make_list(&[1, 2]);
        let mut b = make_list(&[3, 4]);
        a.append(&mut b);
        assert_eq!(a.iter().cloned().collect::<Vec<_>>(), vec![1, 2, 3, 4]);
        assert!(b.is_empty());
    }

    #[test]
    fn append_empty_to_nonempty() {
        let mut a = make_list(&[1, 2]);
        let mut b: LinkedList<i32> = LinkedList::new();
        a.append(&mut b);
        assert_eq!(a.iter().cloned().collect::<Vec<_>>(), vec![1, 2]);
        assert!(b.is_empty());
    }

    #[test]
    fn append_nonempty_to_empty() {
        let mut a: LinkedList<i32> = LinkedList::new();
        let mut b = make_list(&[3, 4]);
        a.append(&mut b);
        assert_eq!(a.iter().cloned().collect::<Vec<_>>(), vec![3, 4]);
        assert!(b.is_empty());
    }

    #[test]
    fn iter_mut_next_back_mutates_from_back() {
        let mut list = make_list(&[1, 2, 3]);
        let mut iter = list.iter_mut();
        assert_eq!(iter.next_back(), Some(&mut 3));
        assert_eq!(iter.next_back(), Some(&mut 2));
        assert_eq!(iter.next_back(), Some(&mut 1));
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn iter_next_back_iterates_from_back() {
        let list = make_list(&[1, 2, 3]);
        let mut iter = list.iter();
        assert_eq!(iter.next_back(), Some(&3));
        assert_eq!(iter.next_back(), Some(&2));
        assert_eq!(iter.next_back(), Some(&1));
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn into_iter_next_back_iterates_from_back() {
        let list = make_list(&[1, 2, 3]);
        let mut iter = list.into_iter();
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next_back(), Some(2));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn iter_size_hint_and_exact_size() {
        let list = make_list(&[1, 2, 3]);
        let mut iter = list.iter();
        assert_eq!(iter.size_hint(), (3, Some(3)));
        iter.next();
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.len(), 2);
    }

    #[test]
    fn iter_mut_size_hint_and_exact_size() {
        let mut list = make_list(&[1, 2, 3]);
        let mut iter = list.iter_mut();
        assert_eq!(iter.size_hint(), (3, Some(3)));
        iter.next();
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.len(), 2);
    }

    #[test]
    fn into_iter_size_hint_and_exact_size() {
        let list = make_list(&[1, 2, 3]);
        let mut iter = list.into_iter();
        assert_eq!(iter.size_hint(), (3, Some(3)));
        iter.next();
        assert_eq!(iter.size_hint(), (2, Some(2)));
    }
}
