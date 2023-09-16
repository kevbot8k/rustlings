// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

// can either use implicit return like below, or explicit return statement
fn square(num: i32) -> i32 {
    num * num
}
