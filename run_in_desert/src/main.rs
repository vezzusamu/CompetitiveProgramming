
fn main() {
    let fuel = [(2,2) ,(11, 11), (1, 21)];
    
    run_in_desert(&fuel, 22);
}

//Time complexity O(k).   k= number of gas station along the desert
fn run_in_desert(arr: &[(i32,i32)], street_len: i32 ){
    let mut starting_fuel = arr[0].0;
    let mut current_fuel = arr[0].1;
    for i in 1 .. arr.len(){
        if current_fuel - arr[i].0 < 0 {
            starting_fuel += -(current_fuel - arr[i].0);
            current_fuel = 0;
        } else {
            current_fuel -= arr[i].0;
        }
        current_fuel += arr[i].1;
    }
    let last_part_fuel:i32 = street_len - (arr[arr.len() - 1].0 + current_fuel);
    if last_part_fuel > 0 {
        starting_fuel += last_part_fuel;
    }
    println!("Starting fuel {starting_fuel}");
}
    