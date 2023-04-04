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

    let mut remaining_attempts = 3;
    println!("Try to guess this word: {}" , word);
    println!("Type the first character and then move on to the next!");

    for idx in empty_indices {
        if remaining_attempts == 0 {
            println!("You lost!. Try again.");
            break
        }
        println!("Enter the character. This character has {} attempts remaining", remaining_attempts);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let ch: char = input.trim().to_lowercase().chars().nth(0).expect("Failed to convert to character");
        if ch != word_chars[idx] {
            remaining_attempts -= 1;
        }
        else {
            continue
        }
            
    }
       
}
