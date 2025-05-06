
fn is_prime(x: u16) -> bool {
    if x <= 1{
        return false;
    }
    if x!=2 && x%2==0 {
        return false;
    }
    for i in 3..=((x as f64).sqrt() as u16){
        if x % i == 0 {
            return false;
        }
    }
    return true;

}

fn next_prime(x: u16) -> Option<u16>{
    let mut aux = x+1;
    while aux < u16::MAX{
        if is_prime(aux){
            return Some(aux)
        }
        aux+=1;
    }
    return None
}
fn main() {
   let mut x:u16=1;
   while let Some(i)=next_prime(x){
    println!("{i}");
    x = i;
   }
}
