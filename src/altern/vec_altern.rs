//! # VecAltern
//!
//! The `vec_altern` module provides an iterator, `VecAltern`, for alternately traversing multiple iterators.
//!
//! ## Usage
//!
//! To use `VecAltern`, create a new instance with `VecAltern::new()` and add iterators using the `add` method (or `add_and` for a build pattern).
//! You can also use the macro `altern` for more concise syntax and little performance optimization.
//! The `next` method will then yield elements from the added iterators in a round-robin fashion until all iterators are exhausted.
//!
//! ## Examples
//!
//! ```rust
//! use combin_iterator::altern::VecAltern;
//! let vec1 = vec![1, 4, 7, 9];
//! let vec2 = vec![2, 5];
//! let vec3 = vec![3, 6, 8];
//!
//! // // Create a VecAltern iterator and add individual iterators, with a build pattern.
//! let mut iter = VecAltern::new();
//! iter.add(vec1.iter());
//! iter.add(vec2.iter());
//! iter.add(vec3.iter());
//!
//! assert_eq!(iter.collect::<Vec<_>>(), vec![&1, &2, &3, &4, &5, &6, &7, &8, &9]);
//!
//! // You can also use a build pattern:
//! let iter = VecAltern::new().add_and(vec1.iter()).add_and(vec2.iter()).add_and(vec3.iter());
//! assert_eq!(iter.collect::<Vec<_>>(), vec![&1, &2, &3, &4, &5, &6, &7, &8, &9]);
//!
//! // Alternatively, use the `altern!` macro for a more concise syntax, and some perfomance optimization
//! use combin_iterator::altern;
//! let iter_macro = altern!(vec1.iter(), vec2.iter(), vec3.iter());
//!
//! // Both iterators should yield the same results
//! assert_eq!(iter_macro.collect::<Vec<_>>(), vec![&1, &2, &3, &4, &5, &6, &7, &8, &9]);
//! ```
//!
//! ## Notes
//!
//! - The `altern!` macro provides a convenient way to create an `Altern` iterator with a cleaner syntax, and
//!   a little performance optimization (with the function: `with_capacity`, like in `Vec`). If you know at
//!   compile time how many iter you will altern between, then use the `altern!` macro.


/// Struct to altern between several iterator
pub struct VecAltern<'a, A> {
    iters: Vec<Box<dyn Iterator<Item = A> + 'a>>,
    current: usize,
}

impl<'a, A> VecAltern<'a, A> {
    /// Creates a new instance of an `Altern` iterator.
    pub fn new() -> Self {
        Self {
            iters: vec![],
            current: 0,
        }
    }

    /// Prepare the capacity, like `vec::with_capacity` does.
    pub fn with_capacity(capacity : usize) -> Self {
        Self {
            iters: Vec::with_capacity(capacity),
            current: 0
        }
    }

    /// Adds an iterator to the `Altern` instance.
    ///
    /// # Arguments
    ///
    /// * `iterator` - An iterator of references to elements of type `A`.
    ///
    /// # Returns
    ///
    /// The updated `Altern` instance with the added iterator, to use like a builder.
    pub fn add_and(mut self, iterator: impl Iterator<Item = A> + 'a) -> Self {
        self.iters.push(Box::new(iterator));
        self
    }

    /// Adds an iterator to the `Altern` instance.
    ///
    /// # Arguments
    ///
    /// * `iterator` - An iterator of references to elements of type `A`.
    ///
    /// # Returns
    pub fn add(&mut self, iterator: impl Iterator<Item = A> + 'a){
        self.iters.push(Box::new(iterator));
    }
}

impl<'a, A> Iterator for VecAltern<'a, A>
{
    type Item = A;


    /// Returns the next element in the iteration sequence.
    ///
    /// The `next` method alternates between the added iterators in a round-robin fashion.
    fn next(&mut self) -> Option<A> {
        loop {
            if self.iters.is_empty() {
                return None;
            } else {
                let next;
                match self.iters.get_mut(self.current) {
                    Some(iter) => {
                        next = (*iter).next()
                    },
                    None => {
                        panic!("altern.current out of bound for altern.iters ")
                    },
                }

                match next {
                    Some(value) => {
                        self.current = (self.current + 1) % self.iters.len();
                        return Some(value)
                    },
                    None => {
                        let _ = self.iters.remove(self.current);
                        let n = self.iters.len().max(1);
                        self.current = self.current % n;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VecAltern;

    #[test]
    fn vec_altern() {
        let vec1 = vec![1, 4, 7, 9];
        let vec2 = vec![2, 5];
        let vec3 = vec![3, 6, 8];

        let iter = VecAltern::new().add_and(vec1.iter()).add_and(vec2.iter()).add_and(vec3.iter());

        assert_eq!(iter.collect::<Vec<_>>(), vec![&1,&2,&3,&4,&5,&6,&7,&8, &9]);
    }

    #[test]
    fn empty_vec_altern() {
        let vec1 : Vec<u8> = vec![];
        let vec2: Vec<u8> = vec![];
        let vec3: Vec<u8> = vec![];

        let mut iter = VecAltern::new().add_and(vec1.iter()).add_and(vec2.iter()).add_and(vec3.iter());

        assert_eq!(iter.next(), None);
    }

    use crate::altern;
    #[test]
    fn macro_altern() {
        let vec1 = vec![1, 4, 7, 9];
        let vec2 = vec![2, 5];
        let vec3 = vec![3, 6, 8];
        let iter = altern!(vec1.iter(), vec2.iter(), vec3.iter());
        assert_eq!(iter.collect::<Vec<_>>(), vec![&1,&2,&3,&4,&5,&6,&7,&8, &9]);
    }
}