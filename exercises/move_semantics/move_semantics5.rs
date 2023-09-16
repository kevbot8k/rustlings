// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let mut x = 100;    // x = 100
    let y = &mut x;     // y points to same memory address as x
    *y += 100;          // y (and thus x) are set to 200 (100 + 100)
    let z = &mut x;     // z points to same memory addrss as x and y
    *z += 1000;         // we add 1000 to the value at z's memory address thus x,y,z = 1200
    assert_eq!(x, 1200);
}
