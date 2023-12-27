use std::iter::FromIterator;

#[derive(Clone)]
struct Node<T> {
    next: Option<Box<Node<T>>>,
    value: T,
}

pub struct SimpleLinkedList<T> {
    len: usize,
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { len: 0, head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node {
            next: self.head.take(),
            value: element,
        }));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(mut tmp) => {
                self.head = tmp.next.take();
                self.len -= 1;
                return Some(tmp.value);
            }
            None => return None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|v| &v.value)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut reverted = SimpleLinkedList::new();
        let mut next = self.head;
        while let Some(item) = next {
            reverted.push(item.value);
            next = item.next;
        }
        reverted
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut new_list = SimpleLinkedList::new();
        for v in iter.into_iter() {
            new_list.push(v);
        }
        new_list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut new_vec = Vec::with_capacity(linked_list.len);
        let mut next = linked_list.rev().head;
        while let Some(item) = next {
            new_vec.push(item.value);
            next = item.next;
        }
        new_vec
    }
}
