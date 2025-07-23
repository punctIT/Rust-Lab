
fn is_prime(x:u16)->bool{
    if x < 2 || (x %2 == 0 && x != 2){
        return false;
    } 
    let mut d = 3i32;
    while d * d <= x as i32{
        if x as i32 % d ==0 {
            return false;
        }
        d+=1;
    }
    true
}
fn next_prime(x:u16)->Option<u16>{
    let mut n_prime=x+1;
    while n_prime < u16::MAX{
        if is_prime(n_prime){
            return Some(n_prime);
        }
        n_prime+=1
    }
    None
}

fn main() {
    let mut x=1u16;
    while x < u16::MAX {
        if let Some(i) = next_prime(x) {
            println!("{} -> {}", x, i);
            x += 1;
        } else {
            break;
        }
    }
}
