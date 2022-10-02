use std::collections::VecDeque;

fn main() {
    let arr = [2, 1, 1, 4, 1,1 , 2, 3, 6];
    sliding_window_maximum(&arr, 3);
}

fn sliding_window_maximum(arr: &[i32], window_capacity: usize){
    //Value and Order
    let mut deque : VecDeque<(i32, usize)> = VecDeque::new();
    for i in 0 .. arr.len(){
        //Removing the elements to respect the window capacity
        while !deque.is_empty() && i >= deque.back().unwrap().1 + window_capacity {
            deque.pop_back();
        }
        //Removing the elements smaller than the current element from the back of the deque
        while !deque.is_empty() && deque.front().unwrap().0 < arr[i]  {
            deque.pop_front();
        }
        if deque.is_empty() {
            println!("Max is {}", arr[i]);
        } else {
            println!("Max is {}", deque.back().unwrap().0);
        }
        deque.push_front((arr[i], i));
    }
}
    