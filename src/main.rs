extern crate core;

pub struct Person {
    age: Option<i32>,
    name: String
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
    let my_person= Person::from_name(String::from("FirstName"));
    let other_person: Option<Person> = Person::new(String::from("OtherName"), 22);

    match other_person {
        None => println!("Person does exist"),
        Some(i) => match i.age {
            None => println!("User has name {}, but has no age", i.name),
            Some(x) => println!("User has name {}, and has age", x)
        }
    }
}
