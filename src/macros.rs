/// The `altern!` macro provides a convenient syntax for creating an `Altern` iterator.
///
/// # Syntax
///
/// The macro takes a variable number of iterator expressions, separated by commas. It then constructs
/// an `Altern` instance and adds each provided iterator using the `add` method.
///
/// # Examples
///
/// ```
/// use std::vec;
/// use crate::altern;
///
/// let vec1 = vec![1, 4, 7, 9];
/// let vec2 = vec![2, 5];
/// let vec3 = vec![3, 6, 8];
///
/// // Using the altern! macro for concise syntax
/// let iter = altern!(vec1.iter(), vec2.iter(), vec3.iter());
///
/// // The iterator should yield the same results as adding iterators individually
/// assert_eq!(iter.collect::<Vec<_>>(), vec![&1, &2, &3, &4, &5, &6, &7, &8, &9]);
/// ```
#[macro_export]
macro_rules! altern {

    ($($params:expr $(,)?)*) => {
        Altern::new()
        $(
            .add($params)
        )*
    };
}