use std::{io,fs};

#[derive(Debug)]
enum ErrorType{
    To_Many_arguments,
    No_Argument,
    Empty_Line,
}

trait Console{
    fn get_name(&self)->String;
    fn exec(&mut self,x:&[&str])->Result<(),ErrorType>;
}

struct Ping{}
struct Count{}
struct Times{
    count:u32,
}
//struct 

impl Console for Ping{
    fn get_name(&self)->String{
        let s:String=String::from("ping"); 
        return s;
    }
    fn exec(&mut self,x:&[&str])->Result<(),ErrorType>{
        if !x.is_empty(){
            return Err(ErrorType::To_Many_arguments);
        }
        println!("Pong!");
        return Ok(());
    }
}
impl Console for Count{
     fn get_name(&self)->String{
        let s:String=String::from("count"); 
        return s;
    }
    fn exec(&mut self,x:&[&str])->Result<(),ErrorType>{
        if x.is_empty(){
            return Err(ErrorType::No_Argument);
        }
        println!("numar argumente {}",x.len());
        return Ok(());
    }
}
impl Console for Times{
     fn get_name(&self)->String{
        let s:String=String::from("times"); 
        return s;
    }
    fn exec(&mut self,x:&[&str])->Result<(),ErrorType>{
        if !x.is_empty(){
            return Err(ErrorType::To_Many_arguments);
        }
        self.count+=1;
        println!("{}",self.count);
        return Ok(());
    }
}

struct Terminal{
    commands: Vec<Box<dyn Console>>,
}
impl Terminal{
    fn new()->Self{
        Terminal{
            commands: Vec::new(),
        }
    }
    fn register(&mut self,command: Box<dyn Console>)->Result<(),ErrorType>{
        self.commands.push(command);
        return Ok(());
    }
    fn run(&mut self){
        fn read_text(path:&str)->Result<String,io::Error>{
            let s:String=fs::read_to_string(path)?;
            return Ok(s);
        }
        if let Ok(s)=read_text("./resources/text.txt"){
            for i in s.lines(){
                let command: Vec<&str> = i.split_whitespace().collect();
                if command.is_empty() {
                        continue;
                    }
                for i in &mut self.commands{
                    if command[0].to_string()==i.get_name(){
                         match i.exec(&command[1..]){
                            Ok(_c)=>println!("-> Command {} executed succesfully!", command[0]),
                            Err(err) => println!(
                                            "-> Error at executing command {}\n   Possible Reason: {:?}",
                                            command[0], err
                                        ),
                         }
                    }
                }
            }
        }
       
    }
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(Ping {}));
    terminal.register(Box::new(Count {}));
    terminal.register(Box::new(Times { count: 0 }));

    terminal.run();
}


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}