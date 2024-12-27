use std::io;

fn main() {
    println!("Word Counter");
    println!("Enter a sentence:");

    // Read input from the user
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Trim whitespace and split into words
    let word_count: usize = input.trim().split_whitespace().count();

    println!("The input contains {} word(s).", word_count);
}
