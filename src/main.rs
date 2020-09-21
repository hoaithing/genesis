use std::collections::HashMap;
use genesis::show::show_name;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Copy, Clone)]
enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Debug, Clone)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    gender: Gender,
}

impl Person {
    fn new(first_name: String, last_name: String, age: u8) -> Person {
        Person {
            first_name,
            last_name,
            gender: Gender::Male,
            age,
        }
    }

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

    fn check_gender(&self) {
        match self.gender {
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
    }
}

fn apply_to_3<F>(f: F) -> i32
    where F: Fn(i32) -> i32 {
        f(3)
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

    person.check_gender();

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
    println!("{:?}", a);

    let person2 = Person::new(String::from("Hung"), String::from("Ha"), 28);
    println!("{:?}", person2);
    person2.check_gender();
    let mut person3 = person2.clone();
    person3.gender = Gender::Female;
    person3.check_gender();
    person2.check_gender();

    let test = |i: i32| -> i32 { i + 1 };
    let i = 1;
    println!("{}", test(i));
    let haystack = vec![1, 2, 3, 4];
    let element = 1;
    let element1 = 5;

    let contain = |i| haystack.contains(i);
    println!("{}", contain(&element,));
    println!("{}", contain(&element1));
    println!("There are {} element", haystack.len());

    let double = |x| x * 2;

    println!("{}", apply_to_3(double));
}
