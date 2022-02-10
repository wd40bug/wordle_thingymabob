use rand::Rng;
use std::{fs, io::Error};
pub fn get_mystery_word() -> Result<String, Error> {
    let contents = fs::read_to_string("Mystery Words.csv")?;
    let mut mystery_words = Vec::new();
    for line in contents.lines() {
        mystery_words.push(line);
    }
    let x = rand::thread_rng().gen_range(0..mystery_words.len());
    Ok(mystery_words[x].to_string())
}
pub fn get_guessable_words() -> Result<Vec<String>, Error> {
    let contents = fs::read_to_string("Guessable Words.csv")?;
    let mut guessable_words = Vec::new();
    for line in contents.lines() {
        guessable_words.push(line.to_string());
    }
    Ok(guessable_words)
}
