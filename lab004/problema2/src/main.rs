use std::{io,fs};


fn read_text(path:&str)->Result<String,io::Error>{
    let text:String=fs::read_to_string(path)?;
    return Ok(text);
}
fn rot13(s: &mut String){
    let mut s2:String=String::new();
    for i in s.chars(){
        let mut c:char=i;
        if ! i.is_ascii(){
            panic!("Error , not all characters are ascii");
        }
        if i >='A' && i <='Z'{
            if c as u8 + 13 <= 'Z' as u8{
                c = (c as u8 +13)as char;
            }
            else {
                let k:u8 = c as u8 + 13 - 'Z' as u8;
                c= ('A' as u8 +k -1) as char;
            }
        }
        if i >='a' && i <='z'{
            if c as u8 + 13 <= 'z' as u8{
                c = (c as u8 +13)as char;
            }
            else {
                let k:u8 = c as u8 + 13 - 'z' as u8;
                c= ('a' as u8 +k -1) as char;
            }
        }
        s2.push(c);
    }
    *s=s2;
}  
fn main() {
    let mut s:String = read_text("text.txt").unwrap();
    rot13(&mut s);
    println!("{}",s)
}
