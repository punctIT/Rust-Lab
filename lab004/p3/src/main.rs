
use std::{io,fs};


fn read_text(path:&str)->Result<String,io::Error>{
    let text:String=fs::read_to_string(path)?;
    return Ok(text);
}
fn show_repleced_text(text:String){
    let words:Vec<&str> = text.split_whitespace().collect();
    for word in words{
        match word{
            "dl"=>print!("domnul "),
            "pt"|"ptr"=>print!("pentru "),
            "dna"=>print!("doamna "),
            _=>print!("{} ",word),
        }
    }
}
fn main() {
    if let Ok(text)=read_text("text.txt"){
        println!("{}",text);
        show_repleced_text(text);
    }
    else {
        println!("eroare la citire");
    }
}
