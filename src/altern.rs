//! This module contains some structure to altern between iterators
//!
//! ## When to use what ?
//!
//! - `BiAltern`is the most efficient itterator here, in terme of performance, for 2 iterable.<br/>
//!    But, if you want to traverse more than 2 iterator, is drawback is that it require some reflexion, and manipulation to maybe
//!    create the iterator that you want (see the section `Common Mistake` in the module `bi_altern`). <br/>
//!    Futher more, you need to know the number of iterator at compile time for use this one.
//!    He also implement the `std::iter::traits::ExactSizeIterator` and ``std::iter::traits::DoubleEndedIterator` traits if possible.
//! - `VecAltern` is a little less time perfomant iterator (but still good enough for the majority of usage)`, but more flexible.
//!    You can easily iterate over as many iterator you want, and add them at runtime, and during the iteration.
//!
//! ## Performance comparaison
//!
//! Here are the result of benchmarks done to compare the speed of each:
//!
//! - `BiAltern`: <br/>
//!     test create_100000x2           ... bench:           0 ns/iter (+/- 0) <br/>
//!     test create_100000x4           ... bench:           1 ns/iter (+/- 0) <br/>
//!     test create_and_count_100000x2 ... bench:     213,932 ns/iter (+/- 12,216) <br/>
//!     test create_and_count_100000x4 ... bench:     667,491 ns/iter (+/- 82,120) <br/>
//!     test create_and_count_100000x8 ... bench:   2,144,507 ns/iter (+/- 509,807) <br/>
//!     test create_and_count_20000x32 ... bench:   4,385,295 ns/iter (+/- 1,792,604) <br/>
//!     test create_and_count_50000x16 ... bench:   4,757,520 ns/iter (+/- 550,491) <br/>
//! - `VecAltern`: <br/>
//!     test create_100000x2           ... bench:         157 ns/iter (+/- 55) <br/>
//!     test create_100000x4           ... bench:         264 ns/iter (+/- 18) <br/>
//!     test create_and_count_100000x2 ... bench:   1,499,890 ns/iter (+/- 108,683) <br/>
//!     test create_and_count_100000x4 ... bench:   3,138,175 ns/iter (+/- 266,846) <br/>
//!     test create_and_count_100000x8 ... bench:   6,195,610 ns/iter (+/- 457,717) <br/>
//!     test create_and_count_20000x32 ... bench:   4,375,710 ns/iter (+/- 336,647) <br/>
//!     test create_and_count_50000x16 ... bench:   6,058,750 ns/iter (+/- 372,925) <br/>
//!
//! The first number precise the size of each iterator, and the second precise the number of iterator.


pub mod bi_altern;
pub mod vec_altern;

pub use bi_altern::BiAltern;
pub use bi_altern::AlternWith;
pub use vec_altern::VecAltern;