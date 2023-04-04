use rand::prelude::*;

fn main() {
    // Set the word to guess
    let word: &str = "cyber";

    // Convert the word to a vector of characters
    let mut word_chars: Vec<char> = word.chars().collect();
    
    // Find out the word length
    let word_length = word.len();

    // Create indice vector
    let mut indices: Vec<usize> = vec![];
    for (index, ch) in word_chars.iter().enumerate() {
        indices.push(index);
    }

    // Get 60% of the length to mark these characters as empty
    let empty_position_count = (0.60 * word_length as f32).floor() as usize;

    let mut rng  = rand::thread_rng();
    let empty_indices: Vec<usize> = indices.choose_multiple(&mut rng, empty_position_count).cloned().collect();

    for idx in empty_indices.iter() {
        word_chars[*idx] = '_';
    }

    let mut remaining_attempts = 3;
    let mut word_with_missing_chars: String = word_chars.iter().collect();
    println!("Try to guess this word: {}" , word_with_missing_chars);

    for idx in empty_indices.iter() {
        if remaining_attempts == 0 {
            println!("You lost!. Try again.");
            break
        }
        println!("Enter the {}th character. This character has {} attempts remaining", *idx, remaining_attempts);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let ch: char = input.trim().to_lowercase().chars().nth(0).expect("Failed to convert to character");

        if ch != word_chars[*idx] {
            remaining_attempts -= 1;
        }
        else {
            word_chars[*idx] = ch;
            word_with_missing_chars = word_chars.iter().collect();
            println!("Congratulations! You got this letter correct. The word now is {}", word_with_missing_chars);
        }
            
    }
       
}
