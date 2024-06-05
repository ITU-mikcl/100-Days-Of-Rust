use std::io;

fn main() {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("TODO: panic message");

    let mut input: Vec<i8> = Vec::new();
    for s in input_string.split(',') {
        match s.trim().parse::<i8>() {
            Ok(value) => input.push(value),
            Err(_) => println!("Error: Could not parse '{}' as a number", s),
        }
    }

    let mut cur_max: i8 = 0;
    let mut rain_water_amount: i8 = 0;
    let mut tmp_water: i8 = 0;

    for i in 0..input.len() {
        if input[i] >= cur_max {
            cur_max = next_max(i, &input, input[i]);

            rain_water_amount += tmp_water;
            tmp_water = 0;
        } else {
            tmp_water += cur_max - input[i];
        }
    }

    println!("{}", rain_water_amount);
}

fn next_max(i: usize, input: &Vec<i8>, cur: i8) -> i8 {
    let mut cur_max: i8 = 0;

    for j in (i + 1)..input.len() {
        if input[j] >= cur {
            return cur;
        } else if input[j] > cur_max {
            cur_max = input[j];
        }
    }

    return cur_max;
}