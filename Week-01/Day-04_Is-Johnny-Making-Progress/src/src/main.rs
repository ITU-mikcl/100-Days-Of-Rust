use std::io;

fn main() {
    let mut input = Vec::<i32>::new();

    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("Faild to read line!");

    for num in input_str.split(", ") {
        if let Ok(parsed_num) = num.trim().parse() {
            input.push(parsed_num);
        } else {
            println!("Failed to parse: {}", num);
        }
    }

    println!("{}", progress_days(&input));
}

fn progress_days(input: &Vec<i32>) -> i32{
    let mut miles_last_week = 0;
    let mut total_progress_days = -1;

    for &num in input {
        if num > miles_last_week {
            total_progress_days += 1;
        }

        miles_last_week = num;
    }

    return total_progress_days;
}