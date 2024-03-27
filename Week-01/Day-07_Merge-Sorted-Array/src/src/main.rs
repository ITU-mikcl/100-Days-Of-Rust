use std::io;

fn main() {
    let mut input1: String = String::new();
    let mut input2: String = String::new();

    io::stdin().read_line(&mut input1).expect("Falied to read line!");
    io::stdin().read_line(&mut input2).expect("Falied to read line!");

    let mut nums1: Vec<i32> = input1.trim().split(',').map(|s| s.trim().parse().expect("Falied to parse!")).filter(|&n| n > 0).collect();
    let mut nums2: Vec<i32> = input2.trim().split(',').map(|s| s.trim().parse().expect("Falied to parse!")).filter(|&n| n > 0).collect();

    nums1.append(&mut nums2);
    nums1.sort();

    for (i, num) in nums1.iter().enumerate() {
        print!("{}", num);

        if i < nums1.len() - 1 {
            print!(",");
        }
    }
}