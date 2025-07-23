#[derive(Debug)]
enum MyError{
    OverFlow,
}
fn add_check(x:u32,y:u32)-> Result <u32,MyError>{
    if x.checked_add(y).is_some(){
        return Ok(x+y);
    }
    else {
        Err(MyError::OverFlow)
    }
}
fn main() {
    let (a, b):(u32,u32) = (4_000_000_000, 500_000_000);
    let res=add_check(a,b);
    print!("{:?}",res);
    

 }
 