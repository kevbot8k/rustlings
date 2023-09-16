// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    // &str because in memory this is a pointer to a compiled string
    let number: &str = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);

    // isize is controlled by the arch of the computer (this computer is x86_64 so isize = i64)
    let number: isize = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}
