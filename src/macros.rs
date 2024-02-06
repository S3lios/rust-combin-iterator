/// The `altern!` macro provides a convenient syntax for creating an `Altern` iterator.
///
/// # Syntax
///
/// The macro takes a variable number of iterator expressions, separated by commas. It then constructs
/// an `Altern` instance and adds each provided iterator using the `add` method.
///
/// # Example
/// ```
/// use combin_iterator::altern;
/// let vec1 = vec![1, 4, 7, 9];
/// let vec2 = vec![2, 5];
/// let vec3 = vec![3, 6, 8];
/// let iter = altern!(vec1.iter(), vec2.iter(), vec3.iter());
///
/// assert_eq!(iter.collect::<Vec<_>>(), vec![&1, &2, &3, &4, &5, &6, &7, &8, &9]);
/// ```
///
/// # Expand into
///
/// From the code before, altern! expand into:
/// ```
/// let vec1 = vec![1, 4, 7, 9];
/// let vec2 = vec![2, 5];
/// let vec3 = vec![3, 6, 8];
/// let iter = {
///    let capacity = 1 + (1 + (1 + 0));
///    let mut vec_altern = combin_iterator::iterators::VecAltern::with_capacity(
///        capacity,
///    );
///    vec_altern.add(vec1.iter());
///    vec_altern.add(vec2.iter());
///    vec_altern.add(vec3.iter());
///    vec_altern
///};
///
/// assert_eq!(iter.collect::<Vec<_>>(), vec![&1, &2, &3, &4, &5, &6, &7, &8, &9]);
/// ```
#[macro_export]
macro_rules! altern {
    ($($params:expr $(,)?)*) => {
        {
            let capacity = $crate::count_exprs!($($params),*);
            let mut vec_altern = $crate::iterators::VecAltern::with_capacity(capacity);
            $(
                vec_altern.add($params);
            )*
            vec_altern
        }
    };
}

mod __private {
    #[macro_export]
    #[doc(hidden)]
    macro_rules! count_exprs {
        () => { 0 };
        ($_head:expr $(, $tail:expr)*) => {
            1 + $crate::count_exprs!($($tail),*)
        };
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn macro_altern() {
        let vec1 = vec![1, 4, 7, 9];
        let vec2 = vec![2, 5];
        let vec3 = vec![3, 6, 8];

        let iter = altern!(vec1.iter(), vec2.iter(), vec3.iter());
        assert_eq!(iter.collect::<Vec<_>>(), vec![&1,&2,&3,&4,&5,&6,&7,&8, &9]);
    }
}
