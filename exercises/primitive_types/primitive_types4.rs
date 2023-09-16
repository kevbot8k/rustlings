// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // slices are a pointer and a size integer. they are useful for indexing memory
    // ref: https://doc.rust-lang.org/book/ch04-03-slices.html
    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
