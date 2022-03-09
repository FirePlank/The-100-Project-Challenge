use std::io;
use rand::Rng;
use std::io::Write;

fn main() {
    loop {
        let mut answer = String::new();
        print!("Do you want to play hilo or coinflip?: ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut answer) {
            Ok(_) => {
                answer = answer.to_lowercase().trim().to_string();
                if answer == "hilo" {
                    hilo();
                    break;
                } else if answer == "coinflip" {
                    coinflip();
                    break;
                } else {
                    println!("Please type either \"hilo\" or \"coinflip\"");
                }
            }
            Err(_) => {
                println!("Please type either \"hilo\" or \"coinflip\"");
                continue;
            }
        }
    }
}

fn coinflip() {
    println!("\nWelcome to heads or tails!");
    let mut correct = 0;
    loop {
        let mut guess = String::new();
        print!("Heads, tails or stop?: ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut guess) {
            Ok(_) => {
                guess = guess.to_lowercase().trim().to_string();
                if guess == "stop" { break; }
                if guess != "tails" && guess != "heads" {
                    println!("Please type either \"heads\", \"tails\" or \"stop\"");
                    continue;
                }
                let flip: u32 = rand::thread_rng().gen_range(1..3);
                if flip == 1 {
                    if guess == "heads" {
                        correct += 1;
                        println!("It's heads! You win! Your current win streak is {}", correct);
                    } else {
                        correct = 0;
                        println!("It's heads! You lost your win streak.");
                    }
                } else {
                    if guess == "tails" {
                        correct += 1;
                        println!("It's tails! You win! Your current win streak is {}", correct);
                    } else {
                        correct = 0;
                        println!("It's tails! You lost your win streak.");
                    }
                }
            }
            Err(_) => {
                println!("Please type either \"heads\", \"tails\" or \"stop\"");
                continue;
            }
        }
    }
    println!("Goodbye for now!");
}

fn hilo() {
    println!("\nWelcome to higher or lower! In this game your goal is to guess a number between 1 and 100. You will be told if your guess is too high or too low.");
    let number = rand::thread_rng().gen_range(1..101);
    let mut guesses = 1;
    loop {
        let mut guess = String::new();
        print!("Please guess a number between 1 and 100: ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut guess) {
            Ok(_) => {
                let guess: u32 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a number that is between 1 and 100.");
                        continue;
                    }
                };
                if guess < 1 || guess > 100 {
                    println!("Please enter a number that is between 1 and 100.");
                    continue;
                }
                if guess == number {
                    println!("You guessed the number! It took you {} guesses.", guesses);
                    break;
                } else if guess > number {
                    println!("You guessed high. Guess again!");
                } else {
                    println!("You guessed low. Guess again!");
                }
                guesses+=1;
            }
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        }
    }
    println!("Goodbye for now!");
}