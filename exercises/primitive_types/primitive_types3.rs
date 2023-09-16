// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.


fn main() {
    // arrays can be initalized with a single value repeated x times [val; rep]
    // ref: https://doc.rust-lang.org/std/primitive.array.html 
    let a: [u8;100] = [1;100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
