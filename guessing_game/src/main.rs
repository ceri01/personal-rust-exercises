use std::io;

fn main() {
    println!("Wellcome in guessing game!");
    
    println!("Insert a guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("Your guess is {guess}");
}
