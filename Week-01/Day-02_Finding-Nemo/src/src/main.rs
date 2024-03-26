use std::io;

fn main() {
    println!("Give me a sentence with Nemo");

    let mut sentence = String::new();

    io::stdin().read_line(&mut sentence).expect("Failed to read sentence");

    let mut nemo_was_found= false;
    let mut counter = 0;

    for word in sentence.split_whitespace() {
        counter += 1;
        if word == "Nemo" {
            println!("I found Nemo at {counter}!");
            nemo_was_found = true;
        }
    }

    if !nemo_was_found {
        println!("I can't find Nemo :(");
    }
}