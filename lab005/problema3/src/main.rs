use serde_derive::Deserialize;
use std::{fs, io};

#[derive(Debug, Deserialize,Clone)]
struct Student{
    name : String,
    number: String,
    age:u32,
}
fn read_file_content(file_path: &str) -> Result<String, io::Error> {
    let s = fs::read_to_string(file_path)?;
    Ok(s)
}
fn main() {
    let s=read_file_content("Resurces/text.json").unwrap();

    let mut young_student:Student= Student{
        name:String::from(""),
        number:String::from(""),
        age:u32::MAX,
    };
    let mut old_student:Student= Student{
        name:String::from(""),
        number:String::from(""),
        age:u32::MIN,
    };
    for i in s.lines(){
        let p: Student =  serde_json::from_str(&i).unwrap();
        if old_student.age<p.age{
            old_student=p.clone();
        }
        if young_student.age>p.age{
            young_student=p.clone();
        }
    }
    println!("cel mai mare {:?}",old_student);
    println!("cel mai mic {:?}",young_student);
}
