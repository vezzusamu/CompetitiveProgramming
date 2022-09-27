fn main(){
    let array_to_analyze = [1, 2, 4, -7, 2, -3, 1];
    largest_subarray_sum( &array_to_analyze);
}

fn largest_subarray_sum ( array_to_analyze: &[i32] )  {
    let mut max_sum = 0;
    let mut actual_sum = 0;
    for el in array_to_analyze {
        if actual_sum > 0{
            actual_sum += el;
        } else {
            actual_sum = *el;
        }

        if actual_sum >= max_sum {
            max_sum = actual_sum;
        }
    }
    println!("Max Subbarray sum {}", max_sum);

}