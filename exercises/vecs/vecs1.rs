// vecs1.rs
//
// Your task is to create a `Vec` which holds the exact same elements as in the
// array `a`.
//
// Make me compile and pass the test!
//
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    let v: Vec<i32> = vec![10, 20, 30, 40]; // more ways to do this with Vec::new() and then pushing from a to v or a.to_vec()
    let v: Vec<i32> = a.to_vec();
    let v: Vec<i32> = a.into_iter().collect(); // into_iter iterates by value iter() is by reference and would need to be mapped
    let v: Vec<i32> = a.iter().map(|x| *x).collect();
    //vec!(&a[0..]); would be Vec<&[i32]> and vec!a would be Vec<[i32; 4]>

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
