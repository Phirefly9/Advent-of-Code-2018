extern crate regex;
use std::fs;
use regex::Regex;

fn main() {
    let re = Regex::new(r"(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut fabric_area = [[0; 1000]; 1000];

    for caps in re.captures_iter(&contents) {
        let id = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let x_offset = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let y_offset = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let x_range = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
        let y_range = caps.get(5).unwrap().as_str().parse::<i32>().unwrap();
        

        println!("this is a test {:?}", caps);

        for y_cord in y_offset .. y_offset + y_range {
            for x_cord in x_offset .. x_offset + x_range {
                fabric_area[y_cord as usize][x_cord as usize]+=1;
            }
        }
    }

    let mut count = 0;
    let mut saved_id = 0;
    for yy in 0 .. fabric_area.len() {
        for xx in 0 .. fabric_area.len() {
            if fabric_area[yy as usize][xx as usize] > 1 {
                count+=1;
            }
        }
    }
    println!("The count for part 1 is {}", count);
    println!("The id for part 2 is {}". id)
    

}
