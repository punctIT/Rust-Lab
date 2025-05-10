use std::{io,fs};

fn read_text(path:&str)->Result<String,io::Error>{
    let s:String = fs::read_to_string(path)?;
    Ok(s)
}
fn main() {
    let s:String=read_text("C:/Windows/System32/drivers/etc/hosts").unwrap();
    for i in s.lines(){
        if s.starts_with('#'){
            continue;
        } 
        println!("{} ",i);
    }
}
