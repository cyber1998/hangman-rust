use std::collections::HashMap;
fn main() {
    let word: &str = "cyber";
    let mut word_chars: Vec<char> = word.chars().collect();
    let guess = 'y';
    let guess_position = 1;
    if word_chars[guess_position] == guess {
        println!("Yay, correct guess!");
    }
    else {
        println!("Wrong guess. {} more attempts left.", 1);
    }
}
