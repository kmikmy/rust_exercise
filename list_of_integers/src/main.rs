use std::{io, collections::HashMap};

// return mean from integer_list if integer_list is not empty.
// return None if integer_list is empty.
fn calc_mean(integer_list: &mut Vec<i32>) -> Option<f64> {
    if integer_list.len() == 0 {
        return None;
    }

    let mut sum: i32 = 0;
    for val in integer_list.iter() {
        sum += *val;
    }

    let mean = sum as f64 / integer_list.len() as f64;
    Some(mean)
}

// return median from integer_list if integer_list is not empty.
// return None if integer_list is empty.
fn calc_median(integer_list: &mut Vec<i32>) -> Option<i32> {
    if integer_list.len() == 0 {
        return None;
    }

    integer_list.sort();

    match integer_list.len() % 2 {
        1 => {
            let median_index = integer_list.len() / 2;
            let median = integer_list.get(median_index)
                .expect("Vector's get failed.");
            Some(*median)
        },
        0 => {
            let median_index1 = integer_list.len() / 2 - 1;
            let median_index2 = integer_list.len() / 2;

            let median_left = integer_list.get(median_index1)
                .expect("Vector's get failed.");
            let median_right = integer_list.get(median_index2)
                .expect("Vector's get failed.");
                
            Some((*median_left+*median_right)/2)
        },
        _ => None,
    }

}

// return mode(s) from integer_list if integer_list is not empty.
// return None if integer_list is empty.
fn calc_mode(integer_list: &Vec<i32>) -> Option<Vec<i32>> {
    if integer_list.len() == 0 {
        return None;
    }

    let mut frequency_map: HashMap<i32, u32> = HashMap::new();
    for integer in integer_list {
        let frequency = frequency_map.entry(*integer).or_insert(0);
        *frequency += 1;
    }

    // mode can be multiple values, so named "modes".
    let mut frequency_modes_tuple: (u32, Vec<i32>) = (0, Vec::new());
    for (key, frequency) in frequency_map.iter() {
        if *frequency > frequency_modes_tuple.0 {
            frequency_modes_tuple.0 = *frequency;
            frequency_modes_tuple.1 = vec![*key];
        } else if *frequency == frequency_modes_tuple.0 {
            frequency_modes_tuple.1.push(*key);
        }
    }    

    Some(frequency_modes_tuple.1)
}

// return integer_list from stdin input
fn get_integer_list_from_input() -> Vec<i32> {
    let mut input = String::new();

    println!("Please enter a space-separated list of integers.");
    io::stdin().read_line(&mut input)
        .expect("failed to read_line");

    let mut integer_list: Vec<i32> = Vec::new();
    for word in input.split_whitespace() {
        let integer: i32 = word.parse()
            .expect("input contains a word which cannot be parsed into an integer.");
        integer_list.push(integer);
    }

    integer_list
}

fn main() {
    let mut integer_list = get_integer_list_from_input();

    if let Some(mean) = calc_mean(&mut integer_list) {
        println!("The value of mean of the list is {mean}.");
    }

    if let Some(median) = calc_median(&mut integer_list) {
        println!("The value of median in the list is {median}.");
    }

    if let Some(modes) = calc_mode(&mut integer_list) {
        println!("The value(s) of mode in the list is(are) {modes:?}");
    }
}
