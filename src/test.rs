use crate::assess::assess_letter;
use crate::letter::Letter::*;

use super::*;
#[test]
fn test_determine_word() {
    let guess = "rusty";
    let answer = "query";
    let correct = vec![Almost('r'), Right('u'), Wrong('s'), Wrong('t'), Right('y')];
    assert_eq!(correct, assess_word(guess, answer.to_string()));
    let guess = "russy";
    let answer = "wosty";
    let correct = vec![Wrong('r'), Wrong('u'), Right('s'), Wrong('s'), Right('y')];
    assert_eq!(correct, assess_word(guess, answer.to_string()));
}
#[test]
fn test_determine_letters() {
    let guess = "rusty";
    let answer = "query";
    let correct = vec![Almost('r'), Right('u'), Wrong('s'), Wrong('t'), Right('y')];
    let mut result = Vec::new();
    for (i, letter) in guess.chars().enumerate() {
        result.push(assess_letter((letter, i), &mut answer.to_string()));
    }
    assert_eq!(correct, result);
}
