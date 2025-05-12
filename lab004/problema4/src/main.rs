use std::{io,fs};

fn read_text(path:&str)->Result<String,io::Error>{
    let s:String = fs::read_to_string(path)?;
    Ok(s)
}
fn main() {
    let s:String=read_text("C:/Windows/System32/drivers/etc/hosts").unwrap();
    for line in s.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut columns = line.split_ascii_whitespace();
        if let Some(host) = columns.next() {
            if let Some(id_address) = columns.next() {
                println!("{} => {}", host, id_address);
           }
        }
    }
}
