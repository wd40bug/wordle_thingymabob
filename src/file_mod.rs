use std::fs;
use rand::Rng;
pub fn get_mystery_word() -> &str{
	let contents = fs::read_to_string("Mystery Words.csv")?;
	let mut mystery_words = Vec::new();
	for line in contents.lines(){
		mystery_words.push(line);
	}
	let x = rand::thread_rng().gen_range(0..contents.len());
	contents[x]
}
