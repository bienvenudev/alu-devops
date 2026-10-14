// This function doubles each number in a vector and returns their sum
fn double_and_sum(numbers: Vec<i32>) -> i32 {
    let mut total_sum = 0;

    for num in numbers {
        let doubled = num * 2;
        total_sum += doubled;
    }

    total_sum
}

fn main() {
    println!("--- Task 2: Vector Math Operations ---");

    // Create a vector with some numbers
    let my_numbers = vec![1, 2, 3, 4, 5];
    println!("Original numbers: {:?}", my_numbers);

    // Call the function and grab the result
    let result = double_and_sum(my_numbers);
    
    println!("The sum of the doubled numbers is: {}", result);
}
