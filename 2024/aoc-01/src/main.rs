use std::{collections::HashMap, fs};

fn main() {
    let file_path = "data.txt";

    let content = fs::read_to_string(file_path);

    let mut left_items: Vec<i32> = Vec::new();
    let mut right_items: Vec<i32> = Vec::new();

    for line in content.unwrap().lines() {
        let left = match line.split("   ").nth(0) {
            Some(value) => value.trim().parse::<i32>().unwrap(),
            None => panic!("Error"),
        };

        let right = match line.split("   ").nth(1) {
            Some(value) => value.trim().parse::<i32>().unwrap(),
            None => panic!("Error"),
        };

        left_items.push(left);
        right_items.push(right);
    }

    left_items.sort();
    right_items.sort();

    if left_items.len() != right_items.len() {
        panic!("Error");
    }

    let mut total: i32 = 0;
    for i in 0..left_items.len() {
        total += (left_items[i] - right_items[i]).abs();
    }

    println!("Total res 1: {}", total);

    let mut map: HashMap<i32, i32> = HashMap::new();

    for item in left_items.iter() {
        map.insert(*item, 0);
    }

    for item in right_items.iter() {
        if map.contains_key(item) {
            let value = map.get(item).unwrap();
            map.insert(*item, value + 1);
        }
    }

    let mut total_2: i32 = 0;

    for (key, value) in map.iter() {
        total_2 += key * value;
    }

    println!("Total res 2: {}", total_2);
}
