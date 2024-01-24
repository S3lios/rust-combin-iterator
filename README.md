# combin-iterator
The combin-iterator crate provides a set of combinators for creating iterators through combinations of existing iterators. It aims to simplify the process of composing and chaining iterators to create more complex iteration patterns.

## Features
Combinator Macros: Use convenient macros to combine multiple iterators and create complex iteration sequences.

Round-Robin Alternation: Use the Altern iterator to alternate between elements produced by multiple iterators in a round-robin fashion.

## Usage
rust
```
Copy code
use combin_iterator::{altern, zip, chain};

fn main() {
    let vec1 = vec![1, 4, 7, 9];
    let vec2 = vec![2, 5];
    let vec3 = vec![3, 6, 8];

    // Use the altern! macro for alternating iteration
    let altern_iter = altern!(vec1.iter(), vec2.iter(), vec3.iter());

    // Use the zip! macro for zipping iteration
    let zip_iter = zip!(vec1.iter(), vec2.iter(), vec3.iter());

    // Use the chain! macro for chaining iteration
    let chain_iter = chain!(vec1.iter(), vec2.iter(), vec3.iter());

    // Iterate over the elements
    for element in altern_iter {
        println!("{}", element);
    }

    // Iterate over the zipped elements
    for (a, b, c) in zip_iter {
        println!("{} | {} | {}", a, b, c);
    }

    // Iterate over the chained elements
    for element in chain_iter {
        println!("{}", element);
    }
}
```

## Installation
Add the following dependency to your Cargo.toml:
```
[dependencies]
combin-iterator = "0.1.0"
```

## License
This project is licensed under the MIT License or Apache 2.0 - see the LICENSE file for details.

## Contribution
Contributions are welcome! If you have any ideas, suggestions, or bug reports, please open an issue or submit a pull request.

Happy Iterating with combin-iterator!
