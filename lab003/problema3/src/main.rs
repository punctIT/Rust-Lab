
enum MyError{
    OverFlow,
}
fn add_check(x:u32,y:u32)-> Result <u32,MyError>{
    x.checked_add(y).

 fn main() {
     let (a, b):(u32,u32) = (4_000_000_000, 500_000_000);
     let res=add_check(a,b);
     if res.is_ok(){
        print!("{}",res.unwrap());
     }
 }
 