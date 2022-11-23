//This data structure is a custom version of the Lazy Segment tree
//created to solve the min-max problem
//The segment tree is built according to the max values of the ranges
//Standard Lazy Tree algorithm from https://github.com/mission-peace/interview/blob/master/src/com/interview/tree/SegmentTreeMinimumRangeQuery.java

#[derive(Debug)]
pub struct SegTreeLazyValueCap {
    segment_tree: Vec<i32>,
    lazy_tree: Vec<i32>,
}

impl SegTreeLazyValueCap {
    //The new function initializes the segment tree and the lazy tree
    //with zeros
    pub fn new(n: usize) -> Self {
        //Its used a capacity of 2 * the next power of 2 - 1
        //This uses a space complexity in the worst case of O(4n) => O(n)
        let tree_size = 2 * (n.next_power_of_two()) - 1;
        let mut tree = Vec::<i32>::with_capacity(tree_size);
        let mut lazy = Vec::<i32>::with_capacity(tree_size);
        for _ in 0..tree_size {
            tree.push(0);
            lazy.push(0);
        }
        SegTreeLazyValueCap {
            segment_tree: tree,
            lazy_tree: lazy,
        }
    }

    //This function initialize the tree with the actual values, starting from the input array
    pub fn construct_tree(
        &mut self,
        arr: &[i32],
        current_range_low: usize,
        current_range_high: usize,
        pos: usize,
    ) {
        //When a leaf is reached its assigned the actual value
        if current_range_low == current_range_high {
            self.segment_tree[pos] = arr[current_range_low];
            return;
        }
        let mid = (current_range_low + current_range_high) / 2;
        self.construct_tree(arr, current_range_low, mid, Self::left_child(pos));
        self.construct_tree(arr, mid + 1, current_range_high, Self::right_child(pos));

        //Each node must contain the max from its children
        self.segment_tree[pos] = i32::max(
            self.segment_tree[Self::left_child(pos)],
            self.segment_tree[Self::right_child(pos)],
        );
    }

    fn right_child(pos: usize) -> usize {
        2 * pos + 2
    }

    fn left_child(pos: usize) -> usize {
        2 * pos + 1
    }

    pub fn update_segment_tree_range_lazy(
        &mut self,
        update_range_low: usize,
        update_range_high: usize,
        current_range_low: usize,
        current_range_high: usize,
        pos: usize,
        value_cap: i32,
    ) {
        if current_range_low > current_range_high {
            return;
        }
        //If the lazy tree for a pos contains a different value from zero,
        //it means that the value_cap must be propagated to the node and its children in the lazy tree
        if self.lazy_tree[pos] != 0 {
            self.segment_tree[pos] = i32::min(self.lazy_tree[pos], self.segment_tree[pos]);
            //If the node is not a leaf the value_cap must be propagated in the lazy tree,
            //if the children doesn't contain a smaller value_cap
            if current_range_low != current_range_high {
                self.lazy_tree[Self::left_child(pos)] = i32::min(
                    self.lazy_tree[pos],
                    self.segment_tree[Self::left_child(pos)],
                );
                self.lazy_tree[Self::right_child(pos)] = i32::min(
                    self.lazy_tree[pos],
                    self.segment_tree[Self::right_child(pos)],
                );
            }
            self.lazy_tree[pos] = 0;
        }
        //No overlap condition
        if update_range_low > current_range_high || update_range_high < current_range_low {
            return;
        }
        //Total overlap condition
        if update_range_low <= current_range_low && update_range_high >= current_range_high {
            if self.segment_tree[pos] > value_cap {
                self.segment_tree[pos] = value_cap;
                //If the actual node its not a leaf, the value_cap its propagated to the children
                if current_range_low != current_range_high {
                    self.lazy_tree[Self::left_child(pos)] =
                        i32::min(value_cap, self.segment_tree[Self::left_child(pos)]);
                    self.lazy_tree[Self::right_child(pos)] =
                        i32::min(value_cap, self.segment_tree[Self::right_child(pos)]);
                }
            }
            return;
        }

        let mid = (current_range_low + current_range_high) / 2;
        self.update_segment_tree_range_lazy(
            update_range_low,
            update_range_high,
            current_range_low,
            mid,
            Self::left_child(pos),
            value_cap,
        );
        self.update_segment_tree_range_lazy(
            update_range_low,
            update_range_high,
            mid + 1,
            current_range_high,
            Self::right_child(pos),
            value_cap,
        );
        self.segment_tree[pos] = i32::max(
            self.segment_tree[Self::left_child(pos)],
            self.segment_tree[Self::right_child(pos)],
        );
    }

    pub fn range_maximum_query_lazy(
        &mut self,
        query_bound_low: usize,
        query_bound_high: usize,
        current_range_low: usize,
        current_range_high: usize,
        pos: usize,
    ) -> i32 {
        if current_range_low > current_range_high {
            return 0;
        }

        if self.lazy_tree[pos] != 0 {
            self.segment_tree[pos] = self.lazy_tree[pos];
            //If the actual node its not a leaf, the value_cap its propagated to the children
            if current_range_low != current_range_high {
                self.lazy_tree[Self::left_child(pos)] = i32::min(
                    self.lazy_tree[pos],
                    self.segment_tree[Self::left_child(pos)],
                );
                self.lazy_tree[Self::right_child(pos)] = i32::min(
                    self.lazy_tree[pos],
                    self.segment_tree[Self::right_child(pos)],
                );
            }
            self.lazy_tree[pos] = 0;
        }
        //no overlap
        if query_bound_low > current_range_high || query_bound_high < current_range_low {
            return 0;
        }
        //total overlap
        if query_bound_low <= current_range_low && query_bound_high >= current_range_high {
            return self.segment_tree[pos];
        }
        //partial overlap
        let mid = (current_range_high + current_range_low) / 2;
        i32::max(
            self.range_maximum_query_lazy(
                query_bound_low,
                query_bound_high,
                current_range_low,
                mid,
                Self::left_child(pos),
            ),
            self.range_maximum_query_lazy(
                query_bound_low,
                query_bound_high,
                mid + 1,
                current_range_high,
                Self::right_child(pos),
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::SegTreeLazyValueCap;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Error};

    const TEST_PATH: &'static str = "Testset/";

    #[test]
    fn run_all_tests() {
        for i in 0..11 {
            match test_nth_file(i) {
                Ok(_res) => print!("\nTest {} ok", i),
                Err(_res) => print!("\nTest {} error", i),
            }
        }
    }

    fn test_nth_file(input_n: i32) -> Result<(), Error> {
        let path = TEST_PATH.to_owned() + "input" + &input_n.to_string() + ".txt";
        let path_e_o = TEST_PATH.to_owned() + "output" + &input_n.to_string() + ".txt";

        let input = File::open(path)?;
        let expected_output = File::open(path_e_o)?;

        let buffered = BufReader::new(input);
        let buffered_out = BufReader::new(expected_output);

        let mut array: Vec<Vec<i32>> = Vec::<Vec<i32>>::new();
        let mut output_array = Vec::<i32>::new();
        let mut count = 0;
        let mut n = 0;

        for line in buffered.lines() {
            let mut app_array = Vec::<i32>::new();
            for number in line.unwrap().split(char::is_whitespace) {
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
            count += 1;
        }
        for line in buffered_out.lines() {
            let k = line.unwrap().parse::<i32>().unwrap();
            output_array.push(k);
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

        assert_eq!(output_array, check_array);
        Ok(())
    }
}
