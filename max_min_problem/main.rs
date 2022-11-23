use segment_tree_hands_on::SegTreeLazyValueCap;
use std::io;
use std::io::{Error, Read};

fn main() {
    let mut input = String::new();
    //Reading the input line from the user
    io::stdin().read_to_string(&mut input).unwrap();

    match construct_solution_from_input_file(input) {
        Ok(res) => {
            for x in res.iter() {
                println!("{}", x);
            }
        }
        Err(_res) => print!("\nError, while resolving the queries"),
    }
}

fn construct_solution_from_input_file(buffered: String) -> Result<Vec<i32>, Error> {
    let mut array: Vec<Vec<i32>> = Vec::<Vec<i32>>::new();
    let mut n = 0;

    for (count, line) in buffered.lines().enumerate() {
        let mut app_array = Vec::<i32>::new();
        for number in line.split(char::is_whitespace) {
            if count == 0 {
                n = number.trim().parse::<i32>().unwrap();
                break;
            }
            let val = number.trim().parse::<i32>().unwrap();
            app_array.push(val);
        }
        if count != 0 {
            array.push(app_array);
        }
    }

    let mut segment_tree = SegTreeLazyValueCap::new(n as usize);
    segment_tree.construct_tree(&array[0], 0, (n - 1) as usize, 0);

    array.remove(0);
    let mut check_array = Vec::<i32>::new();

    for el in array {
        let lower_bound = (el[1] - 1).try_into().unwrap();
        let upper_bound = (el[2] - 1).try_into().unwrap();
        //Query operation
        if el[0] == 1 {
            //Removing one element because the input query has index from 1 .. n

            let query_result = segment_tree.range_maximum_query_lazy(
                lower_bound,
                upper_bound,
                0,
                (n - 1).try_into().unwrap(),
                0,
            );

            check_array.push(query_result);
        }
        //Update max element operation
        else {
            let value_bound = el[3];
            segment_tree.update_segment_tree_range_lazy(
                lower_bound,
                upper_bound,
                0,
                (n - 1).try_into().unwrap(),
                0,
                value_bound,
            );
        }
    }

    Ok(check_array)
}
