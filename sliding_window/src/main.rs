use std::collections::VecDeque;

fn main() {
    let arr = [1, 2, 3, 1, 4, 5, 2, 3, 6];
    sliding_window_maximum(&arr, 3);
}

fn sliding_window_maximum(arr: &[i32], window_capacity: usize){
    //Value and Order
    let mut deque : VecDeque<(i32,i32)> = VecDeque::new();
    for i in 0 .. arr.len(){
        if !deque.is_empty() && i >= window_capacity {
            let deque_el = deque.pop_front();
            if deque_el.unwrap().1 < (i + window_capacity).try_into().unwrap() {
                deque.push_front(deque_el.unwrap());
            } 
        }
        while !deque.is_empty() {
            let deque_el = deque.pop_back().unwrap();
            if deque_el.0 < arr[i] {
                continue;
            }
            deque.push_back(deque_el);
            println!("Max is {}",deque_el.0);
            break;
        }
        if deque.is_empty() {
            println!("Max is {}",arr[i]);
        }
        deque.push_front((arr[i],i.try_into().unwrap()));
    }
}
    