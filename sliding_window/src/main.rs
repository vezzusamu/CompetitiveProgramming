use std::collections::{BinaryHeap, VecDeque};

fn main() {
    all_algorithms_test_with_array_and_compare_result(
        Vec::from([1, 2, 3, 4, 10, 6, 9, 8, 7, 5]),
        3,
        Vec::from([3, 4, 10, 10, 10, 9, 9, 8]),
    );
    all_algorithms_test_with_array_and_compare_result(
        Vec::from([4, 3, 8, 9, 0, 1]),
        3,
        Vec::from([8, 9, 9, 9]),
    );
    all_algorithms_test_with_array_and_compare_result(
        Vec::from([1, 6, 7, 1, 5, 3, 5, 1, 5 , 7 , 6]),
        3,
        Vec::from([7, 7, 7, 5, 5, 5, 5, 7, 7]),
    );
}

fn all_algorithms_test_with_array_and_compare_result(
    arr: Vec<i32>,
    window_size: usize,
    compare_arr: Vec<i32>,
) {
    assert_eq!( compare_arr,sliding_window_maximum_better(&arr, window_size));
    assert_eq!(compare_arr, sliding_window_maximum_best(&arr, window_size));
    assert_eq!(compare_arr, sliding_window_maximum_brute(&arr, window_size));
}

//Runs in O(n * log n) because the inserting time in the heap is log n
fn sliding_window_maximum_better(arr: &[i32], window_capacity: usize) -> Vec<i32> {
    let mut heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();
    
    let mut max_values = Vec::<i32>::new();
    for i in 0..window_capacity - 1 {
        heap.push((arr[i], i));
    }
    for i in window_capacity - 1..arr.len() {
        let current_window = i + 1 - window_capacity;
        heap.push((arr[i], i));
        if current_window > heap.peek().unwrap().1 {
            heap.pop();
        }
        max_values.push(heap.peek().unwrap().0);
    }
    return max_values;
}

//Runs in O(n^2)
fn sliding_window_maximum_brute(arr: &[i32], window_capacity: usize) -> Vec<i32> {
    let mut max_values = Vec::<i32>::new();
    for i in 0..arr.len() - window_capacity + 1 {
        let mut current_max = arr[i];
        for j in i + 1..i + window_capacity {
            if arr[j] > current_max {
                current_max = arr[j];
            }
        }
        max_values.push(current_max);
    }
    return max_values;
}

//Runs in Θ(n)
fn sliding_window_maximum_best(arr: &[i32], window_capacity: usize) -> Vec<i32> {
    let mut max_values = Vec::<i32>::new();
    //Value and Order
    let mut deque: VecDeque<(i32, usize)> = VecDeque::new();
    for i in 0..arr.len() {
        //Removing the elements to respect the window capacity
        while !deque.is_empty() && i >= deque.back().unwrap().1 + window_capacity {
            deque.pop_back();
        }
        //Removing the elements smaller than the current element from the back of the deque
        while !deque.is_empty() && deque.front().unwrap().0 < arr[i] {
            deque.pop_front();
        }
        deque.push_front((arr[i], i));
        if i + 1 < window_capacity {
            continue;
        }
        if deque.is_empty() {
            max_values.push(arr[i]);
        } else {
            max_values.push(deque.back().unwrap().0);
        }
    }
    return max_values;
}
