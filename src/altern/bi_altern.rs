//! # BiAltern
//!
//! The `bi_altern` module provides an iterator, `BiAltern`, for alternately traversing exactly 2 iterators.
//!
//! ## Usage
//!
//! To use `BiAltern`, create a new instance with `BiAltern::new(Iterator<Item>, Iterator<Item>)`, or
//! from an `Iterable<Item>`, use `Iterable<Item>::altern_with(Iterable<Item>)`.
//! The `next` method will then yield elements from the 2 iterators in a round-robin fashion until both iterators are exhausted.
//! `BiAltern`is more complete than `crate::altern::VecAltern`, and implements `core::iter::traits::exact_size::ExactSizeIterator` if both
//! iterators implement it. Also, if both additionally implement `DoubleEndedIterator`, then so does `BiAltern`.
//!
//! ## Examples
//!
//! ```rust
//! use combin_iterator::altern::BiAltern;
//! let vec1 = vec![1, 3, 5, 6];
//! let vec2 = vec![2, 4];
//!
//! // Create a BiAltern iterator and add individual iterators
//! let iter = BiAltern::new(vec1.iter(), vec2.iter());
//!
//! assert_eq!(iter.collect::<Vec<_>>(), vec![&1, &2, &3, &4, &5, &6]);
//!
//!
//! // You can also build it in a line thanks to the trait:
//! use combin_iterator::altern::AlternWith;
//! let iter = vec1.iter().altern_with(vec2.iter());
//!
//! assert_eq!(iter.collect::<Vec<_>>(), vec![&1, &2, &3, &4, &5, &6]);
//! ```
//! ### Common mistake
//! `BiAltern` like `VecAltern`can be used to iter over more than 2 iterable by combining them with each other,
//! but the number of iterable must be a power of 2. Otherwise the number of pass on each iterable will not be equal (but maybe it is waht you want).
//! And even if there is a number of iterable is a number of two, the order need to be correct to have the result expected.
//!
//! For exemple:
//! ```
//! use combin_iterator::altern::BiAltern;
//! let vec1 = vec![1, 1, 1, 1];
//! let vec2 = vec![2, 2];
//! let vec3 = vec![3, 3, 3];
//!
//! // Here, `iter` will NOT be equivalent to
//! // vec![true, true, false, true ,true, false, true, false, true],
//! // but instead to vec![true, false, true, false, true, false, true, true, true].
//! // We alternate between BiAltern(vec1.iter(), vec2.iter()) and vec3.iter(),
//! // not between vec1.iter(), vec2.iter() and vec3.iter() at the same level.
//! let iter = BiAltern::new(BiAltern::new(vec1.iter(), vec2.iter()), vec3.iter());
//! assert_ne!(iter.collect::<Vec<_>>(), vec![&1, &2, &3, &1, &2, &3, &1, &3, &1]);
//!
//! let iter = BiAltern::new(BiAltern::new(vec1.iter(), vec2.iter()), vec3.iter());
//! assert_eq!(iter.collect::<Vec<_>>(), vec![&1, &3, &2, &3, &1, &3, &2, &1, &1]);
//!
//! // For the exact same reason:
//! use combin_iterator::altern::AlternWith;
//! let iter = vec1.iter().altern_with(vec2.iter()).altern_with(vec3.iter());
//! assert_ne!(iter.collect::<Vec<_>>(), vec![&1, &2, &3, &1, &2, &3, &1, &3, &1]);
//!
//! let iter = vec1.iter().altern_with(vec2.iter()).altern_with(vec3.iter());
//! assert_eq!(iter.collect::<Vec<_>>(), vec![&1, &3, &2, &3, &1, &3, &2, &1, &1]);
//!
//! // Now if we have 4 vec than we want to iterate over, we can at the same level:
//! let vec1 = vec![1, 1];
//! let vec2 = vec![2, 2];
//! let vec3 = vec![3, 3];
//! let vec4 = vec![4, 4];
//!
//! // Notice: Don't use `altern_with`, otherwise they will never be at the same level, and the last iter will always be iter over one time of two.
//! let iter = BiAltern::new(BiAltern::new(vec1.iter(), vec2.iter()), BiAltern::new(vec3.iter(), vec4.iter()));
//!
//! // Here, the order on wich we will iterate is: vec1.iter() -> vec3.iter() -> vec2.iter() -> vec4.iter() -> vec1.iter() -> ...
//! assert_eq!(iter.collect::<Vec<_>>(), vec![&1, &3, &2, &4, &1, &3, &2, &4]);
//!
//! // If you want it to be vec1.iter() -> vec2.iter() -> vec3.iter() -> vec4.iter() -> vec1.iter() -> ...,
//! // then you should construct it like this:
//! let iter = BiAltern::new(BiAltern::new(vec1.iter(), vec3.iter()), BiAltern::new(vec2.iter(), vec4.iter()));
//! assert_eq!(iter.collect::<Vec<_>>(), vec![&1, &2, &3, &4, &1, &2, &3, &4]);
//! ```
//!
//! ## Notes

pub trait AlternWith<T1 : Iterator<Item = A>, A> {
    fn altern_with<T2: Iterator<Item = A>>(self : Self, other: T2) -> BiAltern<T1, T2, A>
    where
        Self: Sized;
}

impl<T1 : Iterator<Item = A>, A> AlternWith<T1, A> for T1 {
    fn altern_with<T2: Iterator<Item = A>>(self : Self, other: T2) -> BiAltern<T1, T2, A>
    where
        Self: Sized {
            BiAltern::new(self, other)
    }
}

pub struct BiAltern<Iter1 : Iterator, Iter2 : Iterator, Item>
where
    Iter1: Iterator<Item = Item>,
    Iter2: Iterator<Item = Item>
{
    iter1 : Option<Iter1>,
    iter2 :  Option<Iter2>,
    next_is_first : bool,
}

impl<Iter1, Iter2, Item> BiAltern<Iter1, Iter2, Item>
where
    Iter1: Iterator<Item = Item>,
    Iter2: Iterator<Item = Item>
{
    /// Creates a new instance of an `BiAltern` iterator.
    pub fn new(iter1 : Iter1, iter2 : Iter2) -> BiAltern<Iter1, Iter2, Item> {
        BiAltern {
            iter1: Some(iter1),
            iter2: Some(iter2),
            next_is_first: true,
        }
    }
}

impl<Iter1: Iterator<Item = Item>, Iter2: Iterator<Item = Item>, Item> Iterator for BiAltern<Iter1, Iter2, Item>
{
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.next_is_first, self.iter1.as_mut(), self.iter2.as_mut()) {
            (true, Some(iter1), Some( iter2)) => {
                self.next_is_first = !self.next_is_first;
                iter1.next().or_else(|| {self.iter1 = None; iter2.next()})
            },
            (false, Some(iter1), Some( iter2)) => {
                self.next_is_first = !self.next_is_first;
                iter2.next().or_else(|| {self.iter2 = None; iter1.next()})
            },
            (true, Some(iter1), None) => iter1.next(),
            (false, Some(iter1), None) => iter1.next(),
            (true, None, Some(iter2)) => iter2.next(),
            (false, None, Some(iter2)) => iter2.next(),
            (true, None, None) => None,
            (false, None, None) => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match (self.iter1.as_ref(), self.iter2.as_ref()) {
            (None, None) => (0, Some(0)),
            (None, Some(iter2)) => iter2.size_hint(),
            (Some(iter1), None) => iter1.size_hint(),
            (Some(iter1), Some(iter2)) => {
                let hint1 = iter1.size_hint();
                let hint2 = iter2.size_hint();
                let upper_bound = match (hint1.1, hint2.1)  {
                    (None, None) => None,
                    (None, Some(_)) => None,
                    (Some(_), None) => None,
                    (Some(u1), Some(u2)) => Some(u1 + u2),
                };
                (hint1.0 + hint2.0, upper_bound)
            },
        }
    }
}

impl<Iter1: Iterator<Item = Item>, Iter2: Iterator<Item = Item>, Item> DoubleEndedIterator for BiAltern<Iter1, Iter2, Item>
where
        Iter1 : DoubleEndedIterator<Item = Item> + ExactSizeIterator,
        Iter2 : DoubleEndedIterator<Item = Item> + ExactSizeIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        match (self.iter1.as_mut(), self.iter2.as_mut()) {
            (None, None) => None,
            (None, Some(iter2)) => iter2.next_back(),
            (Some(iter1), None) => iter1.next_back(),
            (Some(iter1), Some(iter2)) => {
                let n1 = iter1.len() + self.next_is_first.then(|| 0).unwrap_or_else(|| 1);
                let n2 = iter2.len();
                if n2 >= n1 {
                    iter2.next_back()
                } else {
                    iter1.next_back()
                }
            },
        }
    }
}

impl<Iter1: Iterator<Item = Item>, Iter2: Iterator<Item = Item>, Item> ExactSizeIterator for BiAltern<Iter1, Iter2, Item>
where Iter1 : ExactSizeIterator,
      Iter2 : ExactSizeIterator {}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::BiAltern;

    #[test]
    fn normal_usage() {
        let vec1 = vec![1, 3, 5, 6];
        let vec2 = vec![2, 4];

        let iter = BiAltern::new(vec1.iter(), vec2.iter());

        assert_eq!(iter.collect::<Vec<_>>(), vec![&1,&2,&3,&4,&5,&6]);
    }

    #[test]
    fn reverse() {
        let vec1 = vec![1, 3, 5, 6];
        let vec2 = vec![2, 4];

        let iter = BiAltern::new(vec1.iter(), vec2.iter()).rev();

        assert_eq!(iter.collect::<Vec<_>>(), vec![&6,&5,&4,&3,&2,&1]);
    }

    #[test]
    fn next_and_next_back() {
        let vec1 = vec![1, 3, 5, 6];
        let vec2 = vec![2, 4];

        let mut iter = BiAltern::new(vec1.iter(), vec2.iter());

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next_back(), Some(&6));
        assert_eq!(iter.next_back(), Some(&5));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next_back(), Some(&4));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn len() {
        let vec1 = vec![1, 3, 5, 6];
        let vec2 = vec![2, 4];
        let iter = BiAltern::new(vec1.iter(), vec2.iter());
        assert_eq!(iter.len(), 6);
    }
}