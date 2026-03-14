use std::io;

fn main() {
    println!("Guess the number");

    let secret_number: i32 = rand::random_range(1..100);

    loop {
        println!("input your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("go bigger"),
            std::cmp::Ordering::Greater => println!("go smaller"),
            std::cmp::Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
