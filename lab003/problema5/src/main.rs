use std::io;


#[derive(Debug)]
enum ErrorType{
    ZeroDivied,
    NotDigit,
    NotAscii,
    CompuseNumber,
}
fn stringToInt32(s:&String)->Result<i32,ErrorType>{
    let s2 = s.as_bytes();
    for i in 0..s.len(){
        
        if !s2[i].is_ascii(){
            return Err(ErrorType::NotAscii);
        }
        if s2[i] == ' ' as u8 {
            return Err(ErrorType::CompuseNumber)
        }
       
    }
    let mut nr: i32 = 0;
    for c in s.trim().chars() {
        if c < '0' || c > '9'{
            return Err(ErrorType::NotDigit);
        } 
        let d= c as i32;
        nr=nr*10+d-48;
    }
    return Ok(nr);
}
fn main() {
    let mut nr:i32=0;
    loop {
        println!("nr: ");
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Eroare la citirea din tastatură");

        /*
        println!("operatie: ");
        let mut op = String::new();
        io::stdin()
        .read_line(&mut op)
        .expect("Eroare la citirea din tastatură");
        if op.trim() == "a" {
            break;
        }
        */

        
        
        if stringToInt32(&input).is_ok(){
            println!("{}",stringToInt32(&input).unwrap());
        }
        else {
            println!("ee");
        }
    }
}
