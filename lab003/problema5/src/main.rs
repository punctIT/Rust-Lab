use std::io;


#[derive(PartialEq, Debug)]
enum ErrorType{
    ZeroDivied,
    NotDigit,
    NotAscii,
    CompuseNumber,
    Overflow,
    Result,
}
fn string_to_int32(s:&String)->Result<i32,ErrorType>{
    if s.trim() == "result"{
        return Err(ErrorType::Result);
    }
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

fn add_operation(nr1:i32,nr2:i32)-> Result<i32,ErrorType>{
    match nr1.checked_add(nr2) {
        Some(sum) => Ok(sum),
        None => Err(ErrorType::Overflow),
    }
}
fn sub_operation(nr1:i32,nr2:i32)-> Result<i32,ErrorType>{
    match nr1.checked_sub(nr2) {
        Some(sum) => Ok(sum),
        None => Err(ErrorType::Overflow),
    }
}
fn mul_operation(nr1:i32,nr2:i32)-> Result<i32,ErrorType>{
    match nr1.checked_mul(nr2) {
        Some(sum) => Ok(sum),
        None => Err(ErrorType::Overflow),
    }
}
fn div_operation(nr1:i32,nr2:i32)-> Result<i32,ErrorType>{
    if nr2 == 0 {
        return Err(ErrorType::ZeroDivied);
    }
    match nr1.checked_div(nr2) {
        Some(sum) => Ok(sum),
        None => Err(ErrorType::Overflow),
    }
}

fn main() {
    let mut nr:i32=0;
    loop {
        println!("number: / 'Result'   / Last Result:{}",nr);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Eroare la citirea din tastatură");
        
        let mut nr_new:i32=0;
        if string_to_int32(&input).is_ok(){
            nr_new=string_to_int32(&input).unwrap();
        }
        else {
            if string_to_int32(&input).err()==Some(ErrorType::Result){
                println!("{}",nr);
                break;
            }
        }

        println!("operatie: + - / %   exit");
        let mut op = String::new();
        io::stdin()
            .read_line(&mut op)
            .expect("Eroare la citirea din tastatură");

        match op.trim(){
            "exit"=>break,
            "+" => if add_operation(nr,nr_new).is_ok() { nr +=nr_new }
            "-" => if sub_operation(nr,nr_new).is_ok() { nr -=nr_new }
            "/" => if div_operation(nr,nr_new).is_ok() { nr /=nr_new }
            "x" => if mul_operation(nr,nr_new).is_ok() { nr *=nr_new }
            _ => {}
           
        
        }
    }
}
