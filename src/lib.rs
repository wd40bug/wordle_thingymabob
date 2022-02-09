use simple_ansi::cursor;
use std::{io, process};

use crate::{assess::assess_word, letter::LetterCollection, file_mod::get_guessable_words};

pub fn run(answer: &str) {
    let guessable_words = get_guessable_words().expect("Something has gone wrong");
    let mut guesses = 0;
    while guesses != 6 {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");
        guess = guess.to_lowercase();
        cursor::move_up(1);
        cursor::move_to_column(0);
        if guess.trim().len() != 5 {
            println!("Guess must be 5 characters! found {}",guess.trim().len());
            continue;
        }else if !guessable_words.contains(&guess.trim().to_string()){
            println!("I do not recognize your language!"); 
            continue;
        }
        let guess_vec = assess_word(&guess, answer.to_string());
        guess_vec.print();
        guesses += 1;
        guess = guess.trim().to_string();
        if guess == answer {
            println!("You Win!!!");
            process::exit(0);
        }
    }
    println!("you lost :(, the word was {}",answer);
}
pub mod file_mod;
pub mod assess;
pub mod letter;
#[cfg(test)]
mod test;
