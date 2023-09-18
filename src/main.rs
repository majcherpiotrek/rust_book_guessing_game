use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess
            .trim()
            .parse::<i32>()
            .map_err(|_| "Invalid input - must be a number")
            .map(|g| g.cmp(&secret_number))
        {
            Ok(res) => match res {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            },
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
