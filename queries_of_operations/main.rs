use segment_tree_hands_on::QOOSolver;
use std::io;
use std::io::{Error, Read};

fn main() {
    let mut input = String::new();
    //Reading the input line from the user
    io::stdin().read_to_string(&mut input).unwrap();

    match start(input) {
        Ok(res) => {
            for (count, x) in res.iter().enumerate() {
                if count < res.len() - 1 {
                    print!("{} ", x);
                } else {
                    println!("{}", x);
                }
            }
        }
        Err(_res) => print!("\nError, while resolving the query"),
    }
}

fn start(input_buffer: String) -> Result<Vec<i32>, Error> {
    let mut n = 0;
    let mut m = 0;
    let mut values_array = Vec::<i32>::new();
    let mut k_queries_array = Vec::<Vec<i32>>::new();
    let mut m_operations_array = Vec::<Vec<i32>>::new();
    for (count, line) in input_buffer.lines().enumerate() {
        let mut app_array = Vec::<i32>::new();

        for number in line.split(char::is_whitespace) {
            let val = number.trim().parse::<i32>().unwrap();
            app_array.push(val);
        }

        if count == 0 {
            n = app_array[0];
            m = app_array[1];
        } else if count == 1 {
            values_array = app_array;
        } else if count <= (m + 1) as usize {
            m_operations_array.push(app_array);
        } else {
            k_queries_array.push(app_array);
        }
    }

    let mut qoo_solver = QOOSolver::new(n as usize, values_array);
    qoo_solver.assign_queries_operations(k_queries_array, m_operations_array, m as usize);

    Ok(qoo_solver.get_final_array())
}
