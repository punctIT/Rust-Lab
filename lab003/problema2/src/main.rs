
fn add_check(x:u32,y:u32)->u32{
   match x.checked_add(y){
        Some(sum)=>sum,
        None=>panic!("at the disco"),
   }
}
fn mul_check(x:u32,y:u32)->u32{
    match x.checked_mul(y){
         Some(sum)=>sum,
         None=>panic!("at the disco"),
    }
 }

fn main() {
    let (a, b):(u32,u32) = (4_000_000_000, 500_000_000);
    add_check(b, b); 
    mul_check(a,b);
}
