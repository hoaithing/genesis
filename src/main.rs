#![feature(proc_macro_hygiene, decl_macro)]

use std::collections::HashMap;
use genesis::show::show_name;
use std::fs::File;
use std::io::Read;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
#[macro_use] extern crate rocket;


#[derive(Debug)]
enum Gender {
    Male,
    Female,
    Other
}

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: i8,
    gender: Gender,
}

impl Person {
    fn show_info(&self) {
        println!(
            "{} {} is {:?}",
            self.first_name, self.last_name, self.gender
        );
        show_name(&self.first_name)
    }

    fn get_full_name(&self) -> String {
        format!("Fullname is : {} {}", self.first_name, self.last_name)
    }
}

fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[get("/")]
fn root() -> &'static str {
    "Hello rocket web"
}

fn main() {
    let mut person = Person {
        first_name: String::from("Cherry"),
        last_name: String::from("Love"),
        age: 28,
        gender: Gender::Male,
    };
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    person.gender = Gender::Other;
    person.show_info();
    println!("{}", person.get_full_name());
    let mut new_hashmap = HashMap::new();

    new_hashmap.insert(&field_name, &field_value);
    println!("{}", new_hashmap.get(&field_name).unwrap());

    match person.gender {
        Gender::Male => {
            println!("is male");
        }
        Gender::Female => {
            println!("is female")
        }
        _ => {
            println!("GAYYYYYYY")
        }
    }
    let f = File::open("Cherrylove.py");
    let mut res = match f {
        Result::Ok(file) => file,
        Result::Err(error) => {
            println!("Error {}", error);
            let file = File::create("Cherrylove.py").unwrap();
            file
        }
    };
    let mut text = String::from("import os");
    res.read_to_string(&mut text).unwrap();
    println!("{}", text);
    let mut a: Vec<i32> = vec![1, 2, 3];
    a.push(1);
    println!("{:?}",  a);

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Spawn  : {}", i);
            thread::sleep(Duration::from_micros(1));
        }
    });

    handle.join().unwrap();

    for i in 10..15 {
        println!("From main {}", i);
            thread::sleep(Duration::from_micros(1));
    }

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap();
    });
    let rev = rx.recv().unwrap();
    println!("I Got {}", rev);

    let first = String::from("XXX");
    let second = String::from("YYYYYY");
    let longer = longer(first.as_str(), second.as_str());
    println!("string which longer is {}", longer);

    rocket::ignite().mount("/", routes![root]).launch();

}
