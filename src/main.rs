mod file_system;
extern crate core;

pub trait Summary {
    fn sum(&self) -> String;
}

pub struct Person {
    age: Option<i32>,
    name: String
}

impl Summary for Person {
    fn sum(&self) -> String {
        match self.age {
            None => format!("User has name {}, but has no age", self.name),
            Some(age) => format!("User has name {}, and has age {}", self.name, age)
        }
    }
}

impl Person {
    pub fn from_name(name: String) -> Option<Person> {
        if name.eq("anonymous") {
            return None
        }

        return Some(Person{name, age: None})
    }

    pub fn new(name: String, age: i32) -> Option<Person> {
        if age < 20 {
            return None
        }

        return Some(Person{ name, age: Some(age)})
    }
}

fn main() {
    let _my_person= Person::from_name(String::from("FirstName"));

    let other_person: Option<Person> = Person::new(String::from("OtherName"), 22);
    let files: Vec<String> = file_system::list_files();

    match other_person {
        None => println!("Person does exist"),
        Some(i) => println!("{}", i.sum())
    }

    println!("these are my files: {:?}", files);
}
