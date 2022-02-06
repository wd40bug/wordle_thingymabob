use std::{io, process};
use ansi_term::Colour::*; 
use simple_ansi::cursor;

pub fn run(answer:&str){
    let mut guesses = 0;
    while guesses!=6 {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error reading input");
        guess = guess.to_lowercase();
        cursor::move_up(1);
        cursor::move_to_column(0);
        if guess.len()!=7 {
            println!("Too many letters eh?");
            continue;
        }
        let guess_vec = determine_word(&guess, answer);
        guess_vec.print();
        guesses+=1;
        guess = guess.trim().to_string();
        if  guess == answer {
            println!("You Win!!!");
            process::exit(0);
        }
    }
    println!("you lost :(")
}

#[derive(Debug,PartialEq,Eq)]
pub enum Letter {
    Right(char),
    Wrong(char),
    Almost(char),
}
pub trait LetterCollection {
    fn equals(&self, other:&Vec<Letter>)->bool;
    fn print(&self);
}
impl LetterCollection for Vec<Letter> {
    fn equals(&self, other:&Vec<Letter>)->bool{
        if self.len()!=other.len(){
            return false;
        }
        for (i,letter) in self.iter().enumerate(){
            if letter != &other[i]{
                return false;
            }
        }
        true
    }

    fn print(&self) {
        use Letter::*;
        for letter in self{
            print!("{}",
                match letter{
                    Right(x)=>Green.paint(x.to_string()),
                    Wrong('\n')=>White.paint('\n'.to_string()),
                    Wrong(x)=>Red.paint(x.to_string()),
                    Almost(x)=>Yellow.paint(x.to_string()),
                }
            );

        }
        
    }
}
pub fn determine_letter((letter,i):(char,usize),answer:&str) ->Letter{
    if answer.contains(letter){
        if answer.chars().nth(i).unwrap()==letter {
            return Letter::Right(letter);
        }else{
            return Letter::Almost(letter);
        }
    }else{
        return Letter::Wrong(letter);
    }
}
pub fn determine_word(guess:&str,answer:&str) -> Vec<Letter>{
    let mut result = Vec::new();
    for (i,letter) in guess.chars().enumerate(){
        result.push(determine_letter((letter,i),answer));
    }
    result
}
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_determine_word(){
        use Letter::*;
        let guess = "rusty";
        let answer = "query";
        let correct = vec![Almost('r'),Right('u'),Wrong('s'),Wrong('t'),Right('y')];
        assert!(correct.equals(&determine_word(guess, answer)));
    }
    #[test]
    fn test_eqals(){
        use Letter::*;
        assert!(!vec![Right('a'),Right('a'),Right('a')].equals(&vec![Right('b')]));
        assert!(vec![Right('a'),Right('a'),Right('a')].equals(&vec![Right('a'),Right('a'),Right('a')]))

    }
    #[test]
    fn test_determine_letters(){
        use Letter::*;
        let guess = "rusty";
        let answer = "query";
        let correct = vec![Almost('r'),Right('u'),Wrong('s'),Wrong('t'),Right('y')];
        let mut result = Vec::new();
        for (i,letter) in guess.chars().enumerate(){
            result.push(determine_letter((letter,i),answer));
        }
        assert!(correct.equals(&result));
    }
}