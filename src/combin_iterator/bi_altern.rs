

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
where Iter1 : DoubleEndedIterator<Item = Item> + ExactSizeIterator,
      Iter2 : DoubleEndedIterator<Item = Item> + ExactSizeIterator {}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::BiAltern;

    #[test]
    fn bi_altern() {
        let vec1 = vec![1, 3, 5, 6];
        let vec2 = vec![2, 4];

        let iter = BiAltern::new(vec1.iter(), vec2.iter());

        assert_eq!(iter.collect::<Vec<_>>(), vec![&1,&2,&3,&4,&5,&6]);
    }

    #[test]
    fn bi_altern_reverse() {
        let vec1 = vec![1, 3, 5, 6];
        let vec2 = vec![2, 4];

        let iter = BiAltern::new(vec1.iter(), vec2.iter()).rev();

        assert_eq!(iter.collect::<Vec<_>>(), vec![&6,&5,&4,&3,&2,&1]);
    }

    #[test]
    fn bi_altern_next_and_next_back() {
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
    fn bi_altern_len() {
        let vec1 = vec![1, 3, 5, 6];
        let vec2 = vec![2, 4];

        let iter = BiAltern::new(vec1.iter(), vec2.iter());

        assert_eq!(iter.len(), 6);
    }
}