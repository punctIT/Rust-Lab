
use std::{io,fs};

fn read_text(path:&str)->Result<String,io::Error>{
    let text:String=fs::read_to_string(path)?;
    return Ok(text);
}
fn local_hosts()->Result<(),std::io::Error>{
    let mut text:String=String::new();
    if cfg!(target_os = "windows") {
        text=read_text("text.txt")?;
        //text=read_text("C:\\Windows\\System32\\drivers\\etc\\hosts")?;
    } else if cfg!(target_os = "linux") {
        text=read_text("/etc/hosts")?;
    } else {
        println!("OS unknown");
    }
    for line in text.lines(){
        if !line.is_empty() {
            let chars:Vec<_>=line.chars().collect();
            if chars[0]=='#'{
                continue;
            }
        }
        let words:Vec<_> = line.split_whitespace().collect();
        if words.len()>2{
            println!("localhost=>{}",words[0]);
        }
    }
    Ok(())
}
fn main() {
    if let Err(e)=local_hosts(){
        println!("Err {}",e);
    }
}