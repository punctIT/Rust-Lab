use std::io;
use std::path::{PathBuf};

#[derive(Debug)]
enum ErrorType {
    InvalidPath,
    NotAFile,
}

fn read_path() -> Result<PathBuf, ErrorType> {
    println!("introduceti pathu:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Eroare la citirea din tastatură");

    let input = input.trim().to_string();
    let path = PathBuf::from(input);

    if path.exists() {
        if path.is_file(){
            Ok(path)
        }
        else {
            Err(ErrorType::NotAFile)
        }
    } else {
        Err(ErrorType::InvalidPath)
    }
}
fn print_content(path:PathBuf)->Result<(),std::io::Error>{
    let content = std::fs::read_to_string(path)?;
    println!("{}",content);
    Ok(())
}
fn main() {
    match read_path() {
        Ok(path) => {
            if let Err(e)=print_content(path){
                println!("Errrr {}",e);
            }
        },
        Err(e) => println!("error: {:?}", e),
    }
}
