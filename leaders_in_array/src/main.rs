fn main(){
    let array_to_analyze = [1, 2, 4, 5, 2, 3, 1];
    leaders_in_array( &array_to_analyze);
}

fn leaders_in_array ( array_to_analyze: &[i32] )  {
    let mut maximum_visited = array_to_analyze[array_to_analyze.len() - 1];

    for i in (0 .. array_to_analyze.len()).rev() {

        if array_to_analyze[i] >= maximum_visited {
            println!("Leader found {} in position {}",array_to_analyze[i], i);
            maximum_visited = array_to_analyze[i];
        } 
    }

}