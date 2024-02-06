# combin-iterator
The combin-iterator crate provides a set of combinators for creating iterators through combinations of existing iterators. It aims to simplify the process of composing and chaining iterators to create more complex iteration patterns.

## Features
Combinator Macros: Use convenient macros to combine multiple iterators and create complex iteration sequences.

Round-Robin Alternation: Use the Altern iterator to alternate between elements produced by multiple iterators in a round-robin fashion.

Note: This crate is currently in development, and only little features are yet available.

## Usage
```rust
use combin_iterator::altern;
use combin_iterator::altern::AlternWith;

fn main() {
    let vec1 = vec![1, 4, 7, 9];
    let vec2 = vec![2, 5];
    let vec3 = vec![3, 6, 8];

    // Use the altern! macro for alternating iteration
    let altern_iter = altern!(vec1.iter(), vec2.iter(), vec3.iter());

    // Iterate over the elements
    for element in altern_iter {
        println!("{}", element);
    } // Print: 1 2 3 5 6 7 8 9

    // Or use the AlternWith trait (i.e. combin_iterator::altern::BiAltern)
    let trait_iter = vec1.iter().altern_with(vec2.iter());

    // Iterate over the elements
    for element in trait_iter {
        println!("{}", element);
    } // Print: 1 2 4 5 7 9
}
```

## Changelog


## Installation

Add the following dependency to your Cargo.toml:
```text
[dependencies]
combin-iterator = "0.2.1"
```

## License
This project is licensed under the MIT License or Apache 2.0 - see the LICENSE file for details.

## Contribution
Contributions are welcome! If you have any ideas, suggestions, or bug reports, please open an issue or submit a pull request.

## Notes
- For any concerns related to stability, bug reports, or feature requests, please refer to the crate's
   documentation or contact the crate maintainers for support.
- If you think you find a bug, don't hesitate to contact the crate maintaines. This crate is still in development,
   and while we're doing our best to make sure there aren't any bugs, it's possible that something has missed us.

Happy Iterating with combin-iterator!
