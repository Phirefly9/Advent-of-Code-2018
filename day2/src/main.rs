extern crate itertools;

use std::fs;
use itertools::Itertools;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let chars_map: Vec<Vec<(char, i32)>> = contents.lines()
                        .map(|c| c.chars().sorted().into_iter()
                                  .map(|c| (c, 1))
                                  .coalesce(|(c, n), (d, m)|
                                  if c == d { Ok((c, n + m)) } else { Err(((c, n), (d, m))) })
                                  .collect())
                        .collect();
    let mut two_count = 0;
    let mut three_count = 0;
    for xx in chars_map {
        let mut two_bool = true;
        let mut three_bool = true;
        for yy in xx {
            //print!("({}, {}) ", yy.0, yy.1);
            if yy.1 == 2 && two_bool {
                two_count+=1;
                two_bool = false;
            }
            if yy.1 == 3 && three_bool {
                three_count+=1;
                three_bool = false;
            }
        }
        //println!();
    }

    println!("part 1 Checksum is {}", two_count * three_count);
    
    let chars_list: Vec<Vec<char>> = contents.lines().map(|c| c.chars().collect()).collect();

    for (xx, yy) in chars_list.iter().tuple_combinations() {
        let mut diff_count = 0;
        let mut common = String::new();
        'comp: for (first_char, second_char) in xx.iter().zip(yy.iter()) {
            if first_char == second_char {
                common.push(*first_char);
            } else if diff_count > 1 {
                break 'comp;
            } else {
                diff_count += 1;
            }
        }
        if diff_count == 1 {
            println!("part2 key: {}", common);
            return
        }
    }
}
