fn main(){
    let array_to_analyze = [0, 1, 4, 3, 2];
    missing_number_array( &array_to_analyze, 5);
}

fn missing_number_array ( array_to_analyze: &[i32], number_of_elements: i32)  {
    let summation = calculate_summation(number_of_elements);
    let mut actual_sum = 0;
    for el in array_to_analyze {
        actual_sum += el;
    }
    println!("Missing element {}", summation -  actual_sum);

}

fn calculate_summation(step_number: i32) -> i32 {
    let mut summation = 0;
    for el in 1..step_number + 1{
        summation += el;
    }
    return summation;
}