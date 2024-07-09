use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error: Can't read line");

    let input_number:i8 = input.trim().parse().expect("Must be an integer");

    let mut result = 1.0;

    for i in 2..=input_number {
        result *= (input_number + i) as f32 / i as f32;
    }

    println!("{}", result as u32);
}