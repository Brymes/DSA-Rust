/*
Solution to LeetCode Question
https://leetcode.com/problems/roman-to-integer
*/


use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let mut total_sum: i32 = 0;
    let mut vals = HashMap::new();
    vals.insert("I", 1);
    vals.insert("V", 5);
    vals.insert("X", 10);
    vals.insert("L", 50);
    vals.insert("C", 100);
    vals.insert("D", 500);
    vals.insert("M", 1000);

    let mut chars = s.split("").collect::<Vec<&str>>();
    chars.remove(0);
    chars.remove(chars.len() - 1);
    println!("{:?}", chars);
    let mut index = 0;

    while index < chars.len() {
        let mut current: i32 = 0;
        match vals.get(chars[index]) {
            Some(&number) => {
                current = number;
            }
            _ => {}
        }
        if index + 1 >= chars.len() {
            total_sum += current;
            break;
        }

        match vals.get(chars[index + 1]) {
            Some(&future) => {
                let sum = future - current;
                if future > current {
                    total_sum += sum;
                    index += 1;
                } else {
                    total_sum += current
                }
                println!("Future: {}, Current: {}, Sum: {}, Total: {} ", future, current, sum, total_sum);
            }
            _ => {}
        }
        index += 1;
    }

    total_sum
}
