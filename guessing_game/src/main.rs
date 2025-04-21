use rand::Rng;
use std::cmp::Ordering;
use std::io;

// 1 -- 100
fn main() {
    println!("Guss the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guss = String::new();

        io::stdin()
            .read_line(&mut guss)
            // Result:
            // --- OK(usize)
            // --- Err(io::Error)
            .expect("Failed to read line");

        let guess: u32 = match guss.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!")
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => {
                println!("Too big!");
            }
        }

    }
}
