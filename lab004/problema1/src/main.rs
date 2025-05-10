use std::{io,fs};

fn read_text(path:&str)->Result<String,io::Error>{
    let text:String=fs::read_to_string(path)?;
    return Ok(text);
}

fn main() {
   let s:String=read_text("./Assets/input.txt").unwrap_or_else(|_|String::new());
   let mut max_bytes:usize=usize::MIN;
   let mut max_lenth:usize=usize::MIN;
   let mut bytes_line:String = String::new();
   let mut lentgh_line:String = String::new();
   for i in s.lines(){
        if max_lenth< i.chars().count(){
            max_lenth=i.chars().count();
            lentgh_line=(&i).to_string();
        }
        if max_bytes < i.as_bytes().len(){
            max_bytes= i.as_bytes().len();
            bytes_line = (&i).to_string();
        }
   }
   println!("max bytes:{}",bytes_line);
   println!("max charater:{}",lentgh_line);
}
