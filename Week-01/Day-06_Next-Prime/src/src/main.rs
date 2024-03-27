use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falied to read line!");

    let mut result: usize = input.trim().parse().expect("Failed to parse inpute");

    loop {
        if is_prime(result) {
            break;
        }

        result += 1;
    }

    println!("{}", result);
}

fn is_prime(num: usize) -> bool {
    if num <= 1 {
        return false;
    }

    if num % 2 == 0 {
        return false;
    }

    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }
    
    return true;
}