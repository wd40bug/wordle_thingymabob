use ansi_term::Colour::*;
#[derive(Debug, PartialEq, Eq)]
pub enum Letter {
    Right(char),
    Wrong(char),
    Almost(char),
}
pub trait LetterCollection {
    fn print(&self);
}
impl LetterCollection for Vec<Letter> {
    fn print(&self) {
        use Letter::*;
        for letter in self {
            print!(
                "{}",
                match letter {
                    Right(x) => Green.paint(x.to_string()),
                    Wrong('\n') => White.paint('\n'.to_string()),
                    Wrong(x) => Red.paint(x.to_string()),
                    Almost(x) => Yellow.paint(x.to_string()),
                }
            );
        }
    }
}
