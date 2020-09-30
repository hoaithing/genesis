pub mod extras;
use extras::extras::add;

use serde::{Serialize, Deserialize};
use std::fs::OpenOptions;
use std::io::BufReader;


#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    x: u32,
    y: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomPerson {
    pub name: String,
    pub age: u8,
    pub phones: Vec<String>,
}

static ZERO: i32 = 0;

struct Foo<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

fn get_x_or_zero_ref<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    return if *x > *y {
        x
    } else {
        &ZERO
    }
}

pub fn call_add(a: u32, b: u32) -> u32 {
    add(a, b)
}


pub fn get_text_data() -> CustomPerson {
    let point = Point { x: 1, y: 2 };
    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);
    let file = OpenOptions::new().read(true).open("Cherrylove.json").unwrap();
    let reader = BufReader::new(file);
    let deserialized: CustomPerson = serde_json::from_reader(reader).unwrap();
    deserialized
}
