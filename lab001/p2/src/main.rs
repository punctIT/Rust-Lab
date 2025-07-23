
fn is_coprime(x:i32,y:i32)->bool{
    let mut a=x;
    let mut b=y;
    if a ==0 || b== 0 {
        return true
    }
    while a != b {
        if a > b {
            a-=b;
        }
        else {
            b-=a;
        }
    }
    if a == 1 {
        return true
    }
    false
}
fn print_coprimes(){
    for i in 0..=100{
        for q in 0..=100{
            println!("({},{}) coprime:{}",i,q,is_coprime(i,q));
        }
    }
}

fn main() {
    print_coprimes()
}
