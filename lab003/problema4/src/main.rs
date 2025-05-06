#![allow(unused)]
#[derive(Debug)]
enum ErrorType{
    NullAsciiCharacter,
    NotDigitCharacter,
    NotBase16Digit,
    NotALetter,
    NotPrintable,
}
fn to_uppercase(c:char)->Result<char,ErrorType>{
    if (c > 'z' || c < 'a') && (c > 'Z' || c < 'A') {
        return Err(ErrorType::NotALetter)
    }
    if ! c.is_ascii(){
        return Err(ErrorType:: NullAsciiCharacter);
    }
    let mut f=c;
    if c >= 'a' && c <='z'{
        let mut g = f as u8;
        g -=32;
        f= g as char;
    }
    return Ok(f);
}
fn to_lowercase(c:char)->Result<char,ErrorType>{
    if (c > 'z' || c < 'a') && (c > 'Z' || c < 'A') {
        return Err(ErrorType::NotALetter)
    }
    if ! c.is_ascii(){
        return Err(ErrorType:: NullAsciiCharacter);
    }
    let mut f=c;
    if c >= 'A' && c <='Z'{
        let mut g = f as u8;
        g +=32;
        f= g as char;
    }
    return Ok(f);
}
fn print_char(c:char)->Result<(),ErrorType>{
    if ! c.is_ascii(){
        return Err(ErrorType:: NullAsciiCharacter);
    }
    if c == ' '{
        return Err(ErrorType::NotPrintable);
    }
    println!("{} ",c);
    return Ok(());
}
fn char_to_number(c:char)->Result<u8,ErrorType>{
    if ! c.is_ascii(){
        return Err(ErrorType:: NullAsciiCharacter);
    }
    if c > '9' || c < '0'{
        return Err(ErrorType::NotDigitCharacter);
    }
    let mut aux:u8 = c as u8;
    return Ok(aux-48);
}
//NU E GATA
fn char_to_number_hex(c:char)->Result<u8,ErrorType>{
    if !c.is_ascii(){
        return Err(ErrorType::NullAsciiCharacter);
    }
    return Ok(1);
}
fn print_error(error:ErrorType){
    match error{
        ErrorType::NullAsciiCharacter=>println!("nu e ascii"),
        ErrorType::NotALetter=>println!("nu e litera"),
        ErrorType::NotBase16Digit=>println!("nu e base 16"),
        ErrorType::NotDigitCharacter=>println!("nu e numar "),
        ErrorType::NotPrintable=>println!("nu e afisabil"),
    }
}
fn main() {
    let a=to_uppercase('d');
    print_char('a');
    print_error(ErrorType::NotALetter);
    println!("{}",char_to_number('1').unwrap());
}