
enum ErrorType{
    NotAscii,
    NotADigit,
    NotABase16Digit,
    NotALetter,
    NotPrintable,
}
fn print_error(error:ErrorType){
    match error{
        ErrorType::NotABase16Digit=>println!("Error: is not a base 16 digit"),
        ErrorType::NotADigit=>println!("Error: is not a digit "),
        ErrorType::NotAscii=>println!("Error: not an ascii character"),
        ErrorType::NotALetter=>println!("Error: not a letter"),
        ErrorType::NotPrintable=>println!("Error: not printable"),
    }
}
fn to_uppercase(character:&mut char)->Result<(),ErrorType>{
    let c:char = *character;
    if (c > 'z' || c < 'a') && (c>'Z' || c < 'A'){
        return Err(ErrorType::NotALetter)
    }
    if !c.is_ascii(){
        return Err(ErrorType::NotAscii)
    }
    if c >= 'a' && c <= 'z' {
        *character = (c as u8 - 32) as char;
    }
    Ok(())
}
fn to_lowercase(character:&mut char)->Result<(),ErrorType>{
    let c:char = *character;
    if (c > 'z' || c < 'a') && (c>'Z' || c < 'A'){
        return Err(ErrorType::NotALetter)
    }
    if !c.is_ascii(){
        return Err(ErrorType::NotAscii)
    }
    if c >= 'A' && c <= 'Z' {
        *character = (c as u8 + 32) as char;
    }
    Ok(())
}
fn print_char(character:char)->Result<(),ErrorType>{
    if !character.is_ascii(){
        return Err(ErrorType::NotAscii)
    }
    if character==' '{
        return Err(ErrorType::NotPrintable)
    }
    println!("{}",character);
    Ok(())
}
fn char_to_number(c:char)->Result<u8,ErrorType>{
    if ! c.is_ascii(){
        return Err(ErrorType:: NotAscii);
    }
    if c > '9' || c < '0'{
        return Err(ErrorType::NotADigit);
    }
    let aux:u8 = c as u8;
    return Ok(aux-48);
}
fn char_to_number_hex(c: char) -> Result<u8, ErrorType> {
    if !c.is_ascii() {
        return Err(ErrorType::NotAscii);
    }
    match c {
        '0'..='9' => Ok(c as u8 - '0' as u8),
        'a'..='f' => Ok(c as u8 - 'a' as u8 + 10),
        'A'..='F' => Ok(c as u8 - 'A' as u8 + 10),
        _ => Err(ErrorType::NotABase16Digit),
    }
}

fn main() {
    let mut c = 'a';
    if let Err(e) = to_uppercase(&mut c) {
        print_error(e);
    }
    println!("Uppercase: {}", c);

    if let Err(e) = to_lowercase(&mut c) {
        print_error(e);
    }
    println!("Lowercase: {}", c);

    if let Err(e) = print_char(c) {
        print_error(e);
    }

    match char_to_number('5') {
        Ok(n) => println!("char_to_number('5') = {}", n),
        Err(e) => print_error(e),
    }

    match char_to_number_hex('F') {
        Ok(n) => println!("char_to_number_hex('F') = {}", n),
        Err(e) => print_error(e),
    }

    match char_to_number_hex('g') {
        Ok(n) => println!("char_to_number_hex('g') = {}", n),
        Err(e) => print_error(e),
    }
}