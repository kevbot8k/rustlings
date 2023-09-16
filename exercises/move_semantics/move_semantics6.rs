// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
// this excercise is focused on passing in by address (pointer) or movement by borrowing
// since getchar is getting the last char of data, we change the borrow to an immutable reference 
// allowing for data to be used in subsequent functions
fn get_char(data: &String) -> char {
    (*data).chars().last().unwrap() // would work without the explicit dereference of data
}

// Should take ownership
// since this function is taking ownership of the variable, we keep the borrowed value mutable
// but remove the reference to String
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
