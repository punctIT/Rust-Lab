use std::fs;

#[derive(Debug)]

enum ErrorType {
    ToManyArguments,
    NoArgument,
    EmptyLine,
    UnknowedCommand,
}

trait Console {
    fn get_name(&self) -> String;
    fn exec(&mut self, x: &[&str]) -> Result<(), ErrorType>;
}

struct Ping {}
struct Count {}
struct Times {
    count: u32,
}
struct Sing {}
//struct

impl Console for Ping {
    fn get_name(&self) -> String {
        let s: String = String::from("ping");
        return s;
    }
    fn exec(&mut self, x: &[&str]) -> Result<(), ErrorType> {
        if !x.is_empty() {
            return Err(ErrorType::ToManyArguments);
        }
        println!("Pong!");
        return Ok(());
    }
}
impl Console for Count {
    fn get_name(&self) -> String {
        let s: String = String::from("count");
        return s;
    }
    fn exec(&mut self, x: &[&str]) -> Result<(), ErrorType> {
        if x.is_empty() {
            return Err(ErrorType::NoArgument);
        }
        println!("number of arguments {}", x.len());
        return Ok(());
    }
}
impl Console for Times {
    fn get_name(&self) -> String {
        let s: String = String::from("times");
        return s;
    }
    fn exec(&mut self, x: &[&str]) -> Result<(), ErrorType> {
        if !x.is_empty() {
            return Err(ErrorType::ToManyArguments);
        }
        self.count += 1;
        println!("count = {}", self.count);
        return Ok(());
    }
}
impl Console for Sing {
    fn get_name(&self) -> String {
        let s: String = String::from("sing");
        return s;
    }
    fn exec(&mut self, x: &[&str]) -> Result<(), ErrorType> {
        if !x.is_empty() {
            return Err(ErrorType::ToManyArguments);
        }
        let mut bottels = 99;
        while bottels >= 1 {
            if bottels != 1 {
                if bottels != 0 {
                    println!("{} bottles of tedi on the wall", bottels);
                    println!("{} bottles of tedi", bottels);
                } else {
                    println!("No bottles of tedi on the wall");
                    println!("No bottles of tedi");
                }
            } else {
                println!("{} bottle of tedi on the wall", bottels);
                println!("{} bottle of tedi", bottels);
            }
            println!("Take one down, pass it around");
            bottels -= 1;
            if bottels != 1 {
                if bottels != 0 {
                    println!("{} bottles of tedi on the wall", bottels);
                } else {
                    println!("No bottles of tedi on the wall");
                }
            } else {
                println!("{} bottle of tedi on the wall", bottels);
            }
            println!()
        }
        println!("No bottles of tedi on the wall");
        println!("No bottles of tedi");
        println!("Got to the store , buy some more,");
        println!("99 bottles of tedi on the wall");
        Ok(())
    }
}
struct Terminal {
    commands: Vec<Box<dyn Console>>,
}
impl Terminal {
    fn new() -> Self {
        Terminal {
            commands: Vec::new(),
        }
    }
    fn register(&mut self, command: Box<dyn Console>) {
        self.commands.push(command);
    }
    fn run(&mut self) {
        if let Ok(text) = fs::read_to_string("./resources/text.txt") {
            let lines: Vec<String> = text.lines().into_iter().map(|f| f.to_string()).collect();
            for line in lines {
                let line_command: Vec<&str> = line.split_whitespace().collect();
                if line_command.is_empty() {
                    println!("{:?}", ErrorType::EmptyLine);
                    continue;
                }
                if line_command[0] == "stop" {
                    println!("Program finised");
                    std::process::exit(0);
                }
                let mut command: Vec<_> = self
                    .commands
                    .iter_mut()
                    .filter(|f| line_command[0] == f.get_name())
                    .collect();
                if command.is_empty() {
                    println!("Error {:?}", ErrorType::UnknowedCommand);
                    continue;
                }
                match command[0].exec(&line_command[1..]) {
                    Ok(_c) => println!("-> Command {} executed succesfully!", line_command[0]),
                    Err(err) => println!(
                        "-> Error at executing command {}\n   Possible Reason: {:?}",
                        line_command[0], err
                    ),
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
    terminal.register(Box::new(Sing{}));
    terminal.run();
}
