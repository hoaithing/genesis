pub mod extras;

use extras::extras::add;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    x: u32,
    y: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomPerson {
    pub name: String,
    pub age: u8,
    pub phones: Vec<String>,
}

pub fn number2text(number: u32) -> String {
    let mapping: HashMap<u32, &str> = [
        (0, ""),
        (1, "Một"),
        (2, "Hai"),
        (3, "Ba"),
        (4, "Bốn"),
        (5, "Năm"),
        (6, "Sáu"),
        (7, "Bảy"),
        (8, "Tám"),
        (9, "Chín"),
        (10, "Mười"),
    ]
    .iter()
    .cloned()
    .collect();

    return if number <= 10 {
        mapping.get(&number).unwrap().to_string()
    } else if number > 20 {
        let unit_by_ten = mapping.get(&(&number / 10)).unwrap().to_string();
        let unit_int = number % 10;
        let mut unit = mapping.get(&unit_int).unwrap().to_string();
        if unit == "0" {
            return format!("{} Mươi", unit_by_ten);
        } else if unit_int == 1 {
            unit = String::from("Mốt");
        }
        format!("{} Mươi {}", unit_by_ten, unit)
    } else {
        let unit = mapping.get(&(&number - 10)).unwrap().to_string();
        format!("Mười {}", unit)
    };
}

pub fn call_add(a: u32, b: u32) -> u32 {
    add(a, b)
}

pub fn get_text_data() -> CustomPerson {
    let point = Point { x: 1, y: 2 };
    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);
    let file = OpenOptions::new()
        .read(true)
        .open("Cherrylove.json")
        .unwrap();
    let reader = BufReader::new(file);
    let deserialized: CustomPerson = serde_json::from_reader(reader).unwrap();
    deserialized
}
