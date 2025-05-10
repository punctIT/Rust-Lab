use std::{io,fs};


fn read_text(path:&str)->Result<String,io::Error>{
    let s:String=fs::read_to_string(path)?;
    return Ok(s);
    
}
fn write_text(path:&str,s:&String)->Result<(),io::Error>{
    fs::write(path, &s)?;
    Ok(())
}
fn main() {
    let s:String=read_text("t.txt").unwrap();
    let mut s2:String = String::new();
    let abr:Vec<(String,String)>=vec![
        ("pt".to_string(),"pentru".to_string()),
        ("ptr".to_string(),"pentru".to_string()),
        ("dl".to_string(),"domnul".to_string()),
        ("dna".to_string(),"doamna".to_string())];
  
        for w in s.split(' '){
            //println!("{}",w);
            let mut abrs:bool =false;
            for e in &abr{
                if e.0 == w{
                    abrs=true;
                    s2.push_str(&e.1);
                    break;
                }
            }
            if abrs==false {
                s2.push_str(w);
            }
            s2.push(' ');
        }
    
    write_text("t2.txt",&s2);
}
