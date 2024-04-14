use std::io;
use std::collections::HashMap;

fn main() {
    let map: HashMap<char, &str> = HashMap::from([
        ('2', "abc"),
        ('3', "def"),
        ('4', "ghi"),
        ('5', "jkl"),
        ('6', "mno"),
        ('7', "pqrs"),
        ('8', "tuv"),
        ('9', "wxyz"),
    ]);

    let mut sets: Vec<&str> = Vec::new();
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Falied to read line!");
    let input = input.trim();

    for character in input.chars() {
        sets.push(map.get(&character).unwrap());
    }

    let mut combinations = Vec::new();


    print_combinations(&sets, String::new(), &mut combinations);

    println!("{:?}", combinations);
}

fn print_combinations(sets: &[&str], current: String, combinations: &mut Vec<String>) {
    if !sets.is_empty() {
        let first_set = sets[0];

        for c in first_set.chars() {
            let mut new_current = current.clone();
            new_current.push(c);
            print_combinations(&sets[1..], new_current.clone(), combinations);
        }
    } else {
        combinations.push(current);
    }
}