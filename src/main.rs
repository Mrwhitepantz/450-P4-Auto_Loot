use std::io::{self, BufRead, BufReader};
use std::cmp::max;
use std::time;

fn main() {

    let now = time::Instant::now();
    let stdin = io::stdin();
    let stdin_reader = stdin.lock();
    let mut stdin_buffer = BufReader::new(stdin_reader);
    let mut stdin_string = String::new();

    let _ = stdin_buffer.read_line(&mut stdin_string).unwrap();
    let carry_weight = stdin_string.trim().parse::<u16>().unwrap();
    stdin_string.clear();
    let mut item_name = Vec::<String>::with_capacity(128);
    let mut item_weight = Vec::<u16>::with_capacity(128);
    let mut item_value = Vec::<u16>::with_capacity(128);

    while stdin_buffer.read_line(&mut stdin_string).unwrap() > 0{
        let mut line = stdin_string.split(";");
        item_name.push(line.next().unwrap().to_string());
        item_weight.push(line.next().unwrap().parse::<u16>().unwrap());
        item_value.push(line.next().unwrap().trim().parse::<u16>().unwrap());
        stdin_string.clear();
    }

    let num_items = item_name.len();

    let mut values: Vec<Vec<i32>> = vec![vec![-1;(carry_weight+1).into()];num_items];
    calculate_values((num_items-1).into(), carry_weight.into(), &mut values, &item_weight, &item_value);
    let items = get_items((num_items-1).into(), carry_weight.into(), &mut values, &item_weight, &item_value);

    // for (idx, val) in values.iter().enumerate() {
    //     println!("{:?}",values[idx]);
    // }

    // println!("{:?}", items);

    for item in &items {
        println!("{}, {}, {}", item_name[*item], item_weight[*item], item_value[*item]);
    }
    println!("final weight: {}", items.iter().map(|&i| item_weight[i]).sum::<u16>());
    println!("final value: {}", items.iter().map(|&i| item_value[i]).sum::<u16>());
    println!("time taken in microseconds: {}", now.elapsed().as_micros());
}

fn calculate_values(i: usize, j: usize, values: &mut Vec<Vec<i32>>, item_weights: &Vec<u16>, item_values: &Vec<u16>) {
    if i == 0 || j <= 0 {
        values[i][j] = 0;
        return;
    }
    if values[i-1][j] == -1{
        calculate_values(i-1, j, values, item_weights, item_values);
    }
    if usize::from(item_weights[i]) > j{
        values[i][j] = values[i-1][j];
    } else{
        if values[i-1][j-item_weights[i] as usize] == -1{
            calculate_values(i-1, j-item_weights[i] as usize, values, item_weights, item_values);
        }
        values[i][j] = max(values[i-1][j], values[i-1][j-(item_weights[i] as usize)] + (item_values[i] as i32));
        
    }
}

fn get_items(i: usize, j: usize, mut values: &mut Vec<Vec<i32>>, item_weights: &Vec<u16>, item_values: &Vec<u16>) -> Vec<usize> {
    // println!("i: {}, j: {}", i, j);
    let mut items = Vec::new();
    
    if j == 0 || i == 0{
        if j as isize - item_weights[i] as isize >= 0 {
            items.push(i)
        }
        return items;
    }
    if values[i][j] > values[i-1][j]{
        items.append(&mut get_items(i-1,j-item_weights[i] as usize, values, item_weights, item_values));
        items.push(i);
        return items;
    } else {
        return get_items(i-1, j, values, item_weights, item_values);
    }
}