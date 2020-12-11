mod test_main;

use std::fs::File;
use std::io::{BufReader, BufRead};
use log::{info, debug};
use std::collections::HashMap;
use std::ops::{AddAssign};

fn read_input_data(file_name: &str) -> Vec<i32> {
    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);

    let mut numbers:Vec<i32> = Vec::new();

    for line in f.lines() {
        let line = line.unwrap();
        numbers.push(line.parse().unwrap());
    }

    return numbers;
}

fn prepare_input_array(numbers: &mut Vec<i32>) {
    numbers.push(0);
    numbers.sort();
    numbers.push(numbers[numbers.len()-1]+3);
    debug!("{:?}", numbers);
}

fn solution_part_1(file_name: &str) -> i32 {
    let mut numbers = read_input_data(file_name);
    prepare_input_array(&mut numbers);
    let mut occurrences:HashMap<i32,i32> = HashMap::new();
    occurrences.insert(1,0);
    occurrences.insert(3,0);
    for n in 0..numbers.len()-1 {
        occurrences.get_mut(&(numbers[n+1] - numbers[n])).unwrap().add_assign(1);
    }
    return occurrences[&1]*occurrences[&3];
}

fn solution_part_2(file_name: &str) -> i64 {
    let mut numbers = read_input_data(file_name);
    prepare_input_array(&mut numbers);
    numbers.reverse();
    debug!("{:?}", numbers);
    let mut combinations: HashMap<i32, i64> = HashMap::new();
    combinations.insert(numbers[0], 1);
    for n in 1..numbers.len() {
        combinations.insert(numbers[n],0);
        for k in 1..n+1 {
            if numbers[n-k] - numbers[n] <= 3 {
                let prev_value = *combinations.get(&numbers[n-k]).unwrap();
                combinations.get_mut(&numbers[n]).unwrap().add_assign(prev_value);
            } else {
                break
            }
        }
    }
    debug!("{:?}", combinations);
    return combinations[&0];
}

fn main() {
    env_logger::init();
    info!("{:?}", solution_part_1("inputData.txt"));
    info!("{:?}", solution_part_2("inputData.txt"));
}
