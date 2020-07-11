use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    println!("Please input a number: ");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_number);
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read a line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You did not enter a number");
                continue;
            },
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is too low"),
            Ordering::Greater => println!("Your guess is too high"),
            Ordering::Equal => {
                println!("You guessed correct!!!!");
                break;
            }
        }
    }
        
}
