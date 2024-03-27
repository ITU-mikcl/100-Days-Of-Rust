use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falied to read line!");
    let input_array: Vec<char> = input.chars().filter(|c| c.is_uppercase()).collect();

    let mut alphabet_array: [Vec<char>; 26] = Default::default();

    for character in input_array {
        alphabet_array[character as usize - 'A' as usize].push(character);
    }

    let mut total_sock_pairs = 0;

    for list in alphabet_array.iter() {
        total_sock_pairs += list.len() / 2;
    }

    println!("{}", total_sock_pairs);
}