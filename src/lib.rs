pub mod alternate_iterator;

#[macro_use]
pub mod macros;


fn main() {
    let vec1 = vec![1.0];
    let vec2 = vec![1.0];
    let vec3 = vec![2.0];
    for k in 0..1 {
        print!("{:#?}", k);
    }
}


#[cfg(test)]
mod tests {
}
