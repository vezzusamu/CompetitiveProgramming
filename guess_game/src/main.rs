use std::io;
use rand::Rng;
use std::cmp;


fn main() {
    println!("Guess the number");
    

    let secret_number = rand::thread_rng().gen_range(1..=100);
   

    loop {
        println!("Please insert your number");

        let mut guess = String::new(); 

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        //Shadowing last guess variable
        let guess: u32 = match guess.trim()
        .parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            cmp::Ordering::Less => println!("Too small"),
            cmp::Ordering::Equal => {
                println!("You got it!");
                break;
        },
            cmp::Ordering::Greater => println!("Too big"),
        }
    }
    
}
