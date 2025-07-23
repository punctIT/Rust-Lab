
fn is_prime(x:i32)->bool{
    if x < 2 || (x %2 == 0 && x != 2){
        return false;
    } 
    let mut d = 3;
    while d * d <= x{
        if x % d ==0 {
            return false;
        }
        d+=1;
    }
    true
}
fn print_primes(){
    for i in 0..=100{
        if is_prime(i){
            print!("{} ",i);
        }
    }
}

fn main(){
    print_primes();
}