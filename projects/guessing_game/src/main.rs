use std::io ;
use rand::Rng ;
use std::cmp::Ordering ;

fn main() {
    println!("Guess the number: ");
    println!("Please input your guess.");

    let secret_guess = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number! {}", guess);
                continue;
            },
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_guess) {
            Ordering::Less => println!("Your guess was lower than what expected."),
            Ordering::Greater => println!("Your guess was higher than expected"),
            Ordering::Equal => {
                println!("You won?!?!?!");
                break;
            }, 
        };
    }
}
