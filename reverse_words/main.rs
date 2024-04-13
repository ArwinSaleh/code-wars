fn reverse_word(word: &str) -> String {
    // Pre-allocate memory for the reversed word.
    let mut reversed = String::with_capacity(word.len());
    
    // Reverse the characters and collect them into a String.
    reversed.extend(word.chars().rev());
    
    return reversed
}

fn reverse_words(sentence: &str) -> String {
    // Map each word to its reversed form and collect them into a vector.
    let reversed_words: Vec<String> = sentence
        .split(" ")
        .map(reverse_word)
        .collect();
    
    // Join the reversed words with a space separator.
    return reversed_words.join(" ")
}

fn main() {
    let result: String = reverse_words("double  spaced  words");
    println!("{}", result);
}
