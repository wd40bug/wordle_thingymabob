use crate::letter::Letter;

pub fn assess_letter((letter, i): (char, usize), answer: & mut String) -> Letter {
    if answer.contains(letter) {
        if answer.chars().nth(i).unwrap() == letter {
            answer.replace_range(i..i+1, " ");
            return Letter::Right(letter);
        } else {
            *answer = answer.replacen(letter, " ", 1);
            return Letter::Almost(letter);
        }
    } else {
        return Letter::Wrong(letter);
    }
}
pub fn assess_word(guess: &str, mut answer: String) -> Vec<Letter> {
    let mut result = Vec::new();
    for (i, letter) in guess.chars().enumerate() {
        result.push(assess_letter((letter, i), &mut answer));
    }
    result
}
