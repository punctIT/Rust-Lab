use std::{io,fs};


#[derive(Debug, Clone)]
struct Student{
    name : String,
    number: String,
    age:u32,
}
fn read_text(path:&str)->Result<String,io::Error>{
    let s:String=fs::read_to_string(path)?;
    Ok(s)
}
fn main() {
    let s:String=read_text("Resurces/text.txt").unwrap();

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
        let mut v:Vec<String> = Vec::new();
        for w in i.split(","){
           v.push(w.to_string());
        }
        if let Ok(age)=v[2].parse::<u32>(){
            if age > old_student.age{
                old_student.age=age;
                old_student.number=v[1].clone();
                old_student.name=v[0].clone();
            }
             if age < young_student.age{
                young_student.age=age;
                young_student.number=v[1].clone();
                young_student.name=v[0].clone();
            }
        }
    }
    println!("cel mai mare {:?}",old_student);
    println!("cel mai mic {:?}",young_student);
}
