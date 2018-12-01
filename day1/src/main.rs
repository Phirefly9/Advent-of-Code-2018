use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let digits: Vec<i32> = contents.split("\n").map(|x| x.parse::<i32>().unwrap()).collect();
    let mut cum_sum: i32 = 0;
    let mut prev_vals = HashSet::new();
    prev_vals.insert(cum_sum);

    for xx in digits.iter().cycle() {
        cum_sum += xx;
        if !prev_vals.contains(&cum_sum) {
            prev_vals.insert(cum_sum);
        } else {
            println!("The first repeating value is {}", cum_sum);
            break;
        }
    }
}
