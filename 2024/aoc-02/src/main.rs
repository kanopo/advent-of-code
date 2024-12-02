/*
    The levels are either all increasing or all decreasing.
    Any two adjacent levels differ by at least one and at most three.
*/

use std::{cmp::Ordering, fs};

fn get_data(path: &String) -> Vec<Vec<i32>> {
    let mut data = Vec::new();

    let content = match fs::read_to_string(path) {
        Ok(d) => d,
        Err(_) => {
            panic!("Error while reading file");
        }
    };

    let rows: Vec<&str> = content.trim().split("\n").by_ref().collect();

    for item in rows {
        let nums: Vec<i32> = item
            .trim()
            .split(" ")
            .map(|num| match num.parse::<i32>() {
                Ok(item) => item,
                Err(_) => panic!("Error converting string to u32"),
            })
            .collect();

        data.push(nums);
    }

    return data;
}

fn main() {
    let path = String::from("./test.txt");
    let data = get_data(&path);
    let mut safe_reports = 0;
    'main: for report in &data {
        let mut is_increasing = false;
        let mut is_decreasing = false;

        for i in 0..report.len() - 1 {
            let first = report.get(i).unwrap();
            let second = report.get(i + 1).unwrap();

            let diff = first - second;
            if diff.abs() > 3 {
                continue 'main;
            }

            match diff.cmp(&0) {
                Ordering::Greater => {
                    if is_increasing {
                        continue 'main;
                    }

                    is_decreasing = true;
                }
                Ordering::Less => {
                    if is_decreasing {
                        continue 'main;
                    }

                    is_increasing = true;
                }
                Ordering::Equal => {
                    continue 'main;
                }
            }
        }
        safe_reports += 1;
    }

    println!("safe {}", safe_reports);

    let mut safe_reports = 0;

    'main: for report in data {
        let mut is_increasing = false;
        let mut is_decreasing = false;
        let mut first = -1;
        let mut second = -1;
        let mut jolly = 0;
        let mut jolly_num = 0;
        let len = report.len();
        for (i, item) in report.iter().enumerate() {
            first = match jolly == 1 {
                true => {
                    jolly += 1;
                    jolly_num
                }
                false => report.get(i).unwrap().clone(),
            };
            second = report.get(i).unwrap().clone();
            let diff = first - second;

            if len - i == 2 {
                println!("last {} {:?}", i, item);
                break;
            } else {
                if diff.abs() > 3 && jolly == 0 {
                    jolly_num = first;
                    jolly += 1;
                    continue;
                }
                println!("safe {} {:?}", i, item);
            }

            match diff.cmp(&0) {
                Ordering::Greater => {
                    if is_increasing {
                        continue 'main;
                    }

                    is_decreasing = true;
                }
                Ordering::Less => {
                    if is_decreasing {
                        continue 'main;
                    }

                    is_increasing = true;
                }
                Ordering::Equal => {
                    continue 'main;
                }
            }
        }
        safe_reports += 1;
        break;
    }
    println!("safe {}", safe_reports);
}
