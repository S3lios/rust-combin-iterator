//! # Altern
//!
//! The `Altern` module provides an iterator, `Altern`, for alternately traversing multiple iterators.
//!
//! ## Usage
//!
//! To use `Altern`, create a new instance with `new()` and add iterators using the `add` method. The `next` method
//! will then yield elements from the added iterators in a round-robin fashion until all iterators are exhausted.
//!
//! ## Examples
//!
//! ```rust
//! use std::vec;
//! use crate::altern;
//!
//! let vec1 = vec![1, 4, 7, 9];
//! let vec2 = vec![2, 5];
//! let vec3 = vec![3, 6, 8];
//!
//! // Create an Altern iterator and add individual iterators
//! let iter = Altern::new().add(vec1.iter()).add(vec2.iter()).add(vec3.iter());
//!
//! // Alternatively, use the `altern!` macro for a more concise syntax
//! let iter_macro = altern!(vec1.iter(), vec2.iter(), vec3.iter());
//!
//! // Both iterators should yield the same results
//! assert_eq!(iter.collect::<Vec<_>>(), vec![&1, &2, &3, &4, &5, &6, &7, &8, &9]);
//! assert_eq!(iter_macro.collect::<Vec<_>>(), vec![&1, &2, &3, &4, &5, &6, &7, &8, &9]);
//! ```
//!
//! ## Notes
//!
//! - The `altern!` macro provides a convenient way to create an `Altern` iterator with a cleaner syntax.
//! - The `next` method is designed to be robust, and its internal state management should not lead to panics
//!   under normal usage scenarios. However, if a panic is encountered, it indicates a potential bug or misuse
//!   of the `Altern` iterator. In such cases, it is recommended to contact the crate manager or maintainers
//!   to report the issue and seek assistance in resolving the problem.
//! - For any concerns related to stability, bug reports, or feature requests, please refer to the crate's
//!   documentation or contact the crate maintainers for support.
//!
//! ## License
//!
//! This module is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
//!
//! ## Todo
//!
//! - Add more robustness checks to handle edge cases.
//!
//! ```
//! # #[cfg(test)]
//! # mod tests {
//! #     use std::vec;
//! #     use crate::altern;
//! #     use super::Altern;
//! #     #[test]
//! #     fn todo() {
//! #         todo!("Add robustness check");
//! #     }
//! # }
//! ```

use std::vec;


pub struct Altern<'a, A> {
    iters: Vec<Box<dyn Iterator<Item = &'a A> + 'a>>,
    current: usize,
}

impl<'a, A> Altern<'a, A> {
    /// Creates a new instance of an `Altern` iterator.
    pub fn new() -> Altern<'a, A> {
        Altern {
            iters: vec![],
            current: 0,
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
    /// The updated `Altern` instance with the added iterator.
    pub fn add(mut self, iterator: impl Iterator<Item = &'a A> + 'a) -> Altern<'a, A> {
        self.iters.push(Box::new(iterator));
        self
    }
}

impl<'a, A> Iterator for Altern<'a, A>
{
    type Item = &'a A;


    /// Returns the next element in the iteration sequence.
    ///
    /// The `next` method alternates between the added iterators in a round-robin fashion.
    fn next(&mut self) -> Option<&'a A> {
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
    use std::vec;

    use crate::altern;
    use super::Altern;

    #[test]
    fn altern() {
        let vec1 = vec![1, 4, 7, 9];
        let vec2 = vec![2, 5];
        let vec3 = vec![3, 6, 8];

        let iter = Altern::new().add(vec1.iter()).add(vec2.iter()).add(vec3.iter());

        assert_eq!(iter.collect::<Vec<_>>(), vec![&1,&2,&3,&4,&5,&6,&7,&8, &9]);
    }

    #[test]
    fn macro_combin() {
        let vec1 = vec![1, 4, 7, 9];
        let vec2 = vec![2, 5];
        let vec3 = vec![3, 6, 8];

        let iter = altern!(vec1.iter(), vec2.iter(), vec3.iter());
        assert_eq!(iter.collect::<Vec<_>>(), vec![&1,&2,&3,&4,&5,&6,&7,&8, &9]);


    }

    #[test]
    fn todo() {
        todo!("Add robustness check");
    }
}
