//This data structure solves the Queries of operations [QOO]

#[derive(Debug)]
pub struct QOOSolver {
    //The m_array will be filled with the number of repetitions for each operation
    m_array: Vec<i32>,
    //The increment_array will be added to the values_array and returned
    increment_array: Vec<i32>,
    values_array: Vec<i32>,
    n: usize,
}

impl QOOSolver {
    pub fn assign_queries_operations(
        &mut self,
        k_queries_array: Vec<Vec<i32>>,
        m_operations_array: Vec<Vec<i32>>,
        m: usize,
    ) {
        for _ in 0..m {
            self.m_array.push(0);
        }
        for _ in 0..self.n {
            self.increment_array.push(0);
        }

        //Creating array of m with prefix sum tec
        self.calculate_m_array(k_queries_array, m);

        //Creating array of n increments with prefix sum tec
        self.calculate_increment_array(m_operations_array, m);
    }

    pub fn get_final_array(&self) -> Vec<i32> {
        let mut return_values = Vec::<i32>::new();
        //Getting the final array
        for i in 0..self.n as usize {
            let final_i_value = self.increment_array[i] + self.values_array[i];
            return_values.push(final_i_value);
        }
        return_values
    }

    fn calculate_increment_array(&mut self, m_operations_array: Vec<Vec<i32>>, m: usize) {
        for (i, item) in m_operations_array.iter().enumerate().take(m as usize) {

            let operation_start_position = (item[0] - 1) as usize;
            let operation_end_position = item[1] as usize;
            let mut val = item[2];
            val *= self.m_array[i];
            self.increment_array[operation_start_position] += val;
            if operation_end_position < self.n as usize {
                self.increment_array[operation_end_position] -= val;
            }
        }

        //Extrapolating the actual prefix sum array
        for i in 1..self.increment_array.len() {
            self.increment_array[i] += self.increment_array[i - 1];
        }
    }

    fn calculate_m_array(&mut self, k_queries_array: Vec<Vec<i32>>, m: usize) {
        for query_end_position in k_queries_array {
            let query_start_position = (query_end_position[0] - 1) as usize;
            let el2 = query_end_position[1] as usize;
            self.m_array[query_start_position] += 1;

            if el2 < m as usize {
                self.m_array[el2] -= 1;
            }
        }

        //Extrapolating the actual array from the prefix sum array
        for i in 1..self.m_array.len() {
            self.m_array[i] += self.m_array[i - 1];
        }
    }

    pub fn new(n: usize, values_array: Vec<i32>) -> Self {
        let m_array = Vec::<i32>::new();
        let increment_array = Vec::<i32>::new();

        QOOSolver {
            m_array,
            increment_array,
            values_array,
            n,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::QOOSolver;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Error};

    #[test]
    fn launch_all_tests() {
        for i in 0..3 {
            match test_nth_file(i) {
                Ok(_res) => print!("\nTest {} ok\n", i),
                Err(_res) => print!("\nTest {} error", i),
            }
        }
    }

    const TEST_PATH: &'static str = "Testset/";

    fn test_nth_file(input_n: i32) -> Result<(), Error> {
        let path = TEST_PATH.to_owned() + "input" + &input_n.to_string() + ".txt";
        let path_e_o = TEST_PATH.to_owned() + "output" + &input_n.to_string() + ".txt";

        let input = File::open(path)?;
        let expected_output = File::open(path_e_o)?;

        let buffered = BufReader::new(input);
        let buffered_out = BufReader::new(expected_output);

        let mut output_array = Vec::<i32>::new();
        let mut n = 0;
        let mut m = 0;
        let mut values_array = Vec::<i32>::new();
        let mut k_queries_array = Vec::<Vec<i32>>::new();
        let mut m_operations_array = Vec::<Vec<i32>>::new();
        let mut count = 0;
        for line in buffered.lines() {
            let mut app_array = Vec::<i32>::new();

            for number in line.unwrap().split(char::is_whitespace) {
                let val = number.trim().parse::<i32>().unwrap();
                app_array.push(val);
            }

            if count == 0 {
                n = app_array[0];
                m = app_array[1];
            } else if count == 1 {
                values_array = app_array;
            } else if count <= m + 1 {
                m_operations_array.push(app_array);
            } else {
                k_queries_array.push(app_array);
            }
            count += 1;
        }

        for line in buffered_out.lines() {
            for number in line.unwrap().split(char::is_whitespace) {
                let val = number.trim().parse::<i32>().unwrap();
                output_array.push(val);
            }
        }

        let mut qoo_solver = QOOSolver::new(n as usize, values_array);
        qoo_solver.assign_queries_operations(k_queries_array, m_operations_array, m as usize);

        assert_eq!(qoo_solver.get_final_array(), output_array);

        Ok(())
    }
}
