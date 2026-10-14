// This function checks if a number is even or odd
fn check_even_odd(number: i32) {
    if number % 2 == 0 {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }
}

fn main() {
    println!("--- Task 1: Even or Odd Checker ---");
    
    // Loop through numbers 1 to 10 (inclusive)
    for i in 1..=10 {
        check_even_odd(i);
    }
}
