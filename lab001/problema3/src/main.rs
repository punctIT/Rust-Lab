fn bottelsofteddy(){
    let mut x=99;
    while x >=1 {
        if x != 1 {
            println!("{x} bottles of teddy on the wall");
            println!("{x} bottles of teddy");
        }
        else {
            println!("{x} bottle of teddy on the wall");
            println!("{x} bottle of teddy");
        }
        x-=1;
        println!("Take one down,pass it around");
        if x != 0{
            if x != 1{
                println!("{x} bottles of teddy on the wall");
            }
            else {
                println!("{x} bottle of teddy on the wall");
            }
            
        }
        else {
            println!("No bottles of beer on the wall");
        }
        println!("");
    }
}

fn main() {
    bottelsofteddy();
}
